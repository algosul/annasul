use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use glib::{BoolError, ExitCode, Propagation};
use gtk4::{Application, ApplicationWindow, glib::object::Cast, prelude::*};
use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;

const APP_ID: &str = "com.algosul.rust-serde-type-editor";
const WINDOW_TITLE: &str = "Rust Serde Type Editor (by algosul)";

pub mod de;
pub mod ser;

#[derive(Debug, Error)]
pub enum Error
{
  #[error("Gtk4 Init: {0}")]
  Gtk4Init(BoolError),
  #[error("Failed Exited (Code: {0})")]
  FailedExited(i32),
  #[error("Serializer callback no match, except {except}")]
  SerializerCallbackNoMatch
  {
    except: String
  },
  #[error("Custom Error: {0}")]
  Custom(String),
}

impl serde::ser::Error for Error
{
  fn custom<T>(msg: T) -> Self
  where T: Display
  {
    Self::Custom(msg.to_string())
  }
}

impl serde::de::Error for Error
{
  fn custom<T>(msg: T) -> Self
  where T: Display
  {
    Self::Custom(msg.to_string())
  }
}

pub type Result<T> = std::result::Result<T, Error>;

/// Shared handle to the apply callback: the title bar button and `Editor`
/// both hold a clone.
type OnApply = Rc<RefCell<Option<Box<dyn FnMut()>>>>;

/// Type-erased action run on Apply: deserialize the editor content, transform
/// it with `on_apply`, and re-serialize it back into the window.
type EditAction = Box<dyn FnMut(&ApplicationWindow)>;

/// The variant-switch handler shared with every switcher dropdown.
type SwitchFn = Rc<RefCell<Option<Box<dyn Fn(&str, &str)>>>>;

pub struct Editor
{
  application: Application,
  on_apply:    OnApply,
}

pub struct EditorBuilder
{
  on_apply:    Option<Box<dyn FnMut()>>,
  edit_action: Option<EditAction>,
}

impl Editor
{
  pub fn builder() -> EditorBuilder
  {
    EditorBuilder { on_apply: None, edit_action: None }
  }

  /// Programmatically triggers the user's apply callback (the one registered
  /// with `connect_on_apply`). Note this does not run the type-aware edit
  /// action, which needs the live window and is only reachable through the
  /// title bar button.
  pub fn apply(&self)
  {
    if let Some(on_apply) = self.on_apply.borrow_mut().as_mut()
    {
      on_apply();
    }
  }

  pub fn run(self) -> Result<()>
  {
    let code = self.application.run();
    (code == ExitCode::SUCCESS)
      .ok_or_else(|| Error::FailedExited(code.get() as i32))
  }
}

impl EditorBuilder
{
  /// Edits a value of type `T`. `v` is serialized with `ser::Serializer` and
  /// shown as the window's default content; on Apply the widget tree is
  /// deserialized back into `T`, `on_apply` transforms it, and the result is
  /// re-serialized into the window. Every enum reachable from `T` (nested
  /// included) gets a variant switcher inside its widget box; switching
  /// reconstructs the value with the target variant.
  pub fn edit<T>(
    &mut self, v: T, mut on_apply: impl FnMut(T) -> T + 'static,
  ) -> Result<&mut Self>
  where T: Serialize + DeserializeOwned + 'static
  {
    // Discover every reachable enum type's variant names.
    let registry_rc = Rc::new(RefCell::new(HashMap::new()));
    let _ = T::deserialize(de::DefaultValue::registry(registry_rc.clone()));
    let registry: HashMap<&'static str, &'static [&'static str]> =
      registry_rc.borrow().clone();

    // The switch handler is installed on first render; every dropdown calls it.
    let on_switch: SwitchFn = Rc::new(RefCell::new(None));

    let mut initialized = false;
    self.edit_action = Some(Box::new(move |window: &ApplicationWindow| {
      // First invocation (from `build`) initializes the window with `v`.
      if !initialized
      {
        initialized = true;

        // Switch handler: substitute the target variant from defaults while
        // keeping the other fields' current widget values, then re-render.
        let window_switch = window.clone();
        let registry_switch = registry.clone();
        let on_switch_self = on_switch.clone();
        *on_switch.borrow_mut() = Some(Box::new(move |name, variant| {
          let Some(content) =
            window_switch.child().and_then(|w| w.downcast::<gtk4::Box>().ok())
          else
          {
            return;
          };
          match T::deserialize(de::Deserializer::with_substitute(
            content, name, variant,
          ))
          {
            Ok(value) => match value.serialize(ser::Serializer::None)
            {
              Ok(widgets) =>
              {
                install_switchers(&widgets, &registry_switch, &on_switch_self);
                window_switch.set_child(Some(&widgets));
              }
              Err(e) => eprintln!("edit: failed to serialize variant: {e}"),
            },
            Err(e) =>
            {
              eprintln!("edit: failed to construct variant {variant}: {e}")
            }
          }
        }));

        let content = match v.serialize(ser::Serializer::None)
        {
          Ok(root) => root,
          Err(e) =>
          {
            eprintln!("edit: failed to serialize default value: {e}");
            return;
          }
        };
        install_switchers(&content, &registry, &on_switch);
        window.set_child(Some(&content));
        return;
      }

      // Apply: deserialize the current content, transform, re-serialize.
      let Some(content) =
        window.child().and_then(|widget| widget.downcast::<gtk4::Box>().ok())
      else
      {
        eprintln!("edit: window has no content box");
        return;
      };
      match T::deserialize(de::Deserializer::from_box(content))
      {
        Ok(value) =>
        {
          let value = on_apply(value);
          match value.serialize(ser::Serializer::None)
          {
            Ok(widgets) =>
            {
              // The freshly serialized tree has no switchers; reinstall them.
              install_switchers(&widgets, &registry, &on_switch);
              window.set_child(Some(&widgets));
            }
            Err(e) => eprintln!("edit: failed to re-serialize: {e}"),
          }
        }
        Err(e) => eprintln!("edit: failed to deserialize: {e}"),
      }
    }));
    Ok(self)
  }

  pub fn build(&mut self) -> Result<Editor>
  {
    gtk4::init().map_err(Error::Gtk4Init)?;
    let application = Application::builder().application_id(APP_ID).build();
    let on_apply = Rc::new(RefCell::new(self.on_apply.take()));
    let on_apply_activate = on_apply.clone();
    let edit_action = Rc::new(RefCell::new(self.edit_action.take()));
    let edit_action_activate = edit_action.clone();
    application.connect_activate(move |app| {
      let editor_window = ApplicationWindow::builder()
        .application(app)
        .title(WINDOW_TITLE)
        .visible(false)
        .can_focus(true)
        .default_width(800)
        .default_height(450)
        .build();

      // Apply button lives in the title bar, independent of the content area
      // that `connect_window` manages.
      let on_apply_clicked = on_apply_activate.clone();
      let edit_action_clicked = edit_action_activate.clone();
      let apply_window = editor_window.clone();
      let header = gtk4::HeaderBar::new();
      let apply = gtk4::Button::with_label("Apply");
      apply.connect_clicked(move |_| {
        if let Some(edit) = edit_action_clicked.borrow_mut().as_mut()
        {
          edit(&apply_window);
        }
        if let Some(on_apply) = on_apply_clicked.borrow_mut().as_mut()
        {
          on_apply();
        }
      });
      header.pack_end(&apply);
      editor_window.set_titlebar(Some(&header));

      // First `edit` invocation: initialize the window with the default value.
      if let Some(edit) = edit_action_activate.borrow_mut().as_mut()
      {
        edit(&editor_window);
      }

      editor_window.connect_close_request(|window| {
        window.application().unwrap().quit();
        Propagation::Stop
      });
      editor_window.present();
    });
    Ok(Editor { application, on_apply })
  }
}

/// Walks the widget tree and prepends a variant switcher (`gtk4::DropDown`)
/// to every enum variant box, seeding it with the current variant. `registry`
/// maps enum type names to their variant lists; `on_switch` is invoked with
/// `(enum_name, variant)` when the user picks a different variant.
fn install_switchers(
  gbox: &gtk4::Box, registry: &HashMap<&'static str, &'static [&'static str]>,
  on_switch: &SwitchFn,
)
{
  if let Some((enum_name, variants)) = de::variant_enum_name(gbox)
    .and_then(|name| registry.get(name.as_str()).copied().map(|v| (name, v)))
  {
    let dropdown = gtk4::DropDown::from_strings(variants);
    if let Some(index) = de::current_variant(gbox)
      .and_then(|current| variants.iter().position(|&v| v == current))
    {
      dropdown.set_selected(index as u32);
    }
    let enum_name_cb = enum_name.clone();
    let on_switch_cb = on_switch.clone();
    dropdown.connect_selected_notify(move |dd| {
      let Some(&variant) = variants.get(dd.selected() as usize)
      else
      {
        return;
      };
      if let Some(cb) = on_switch_cb.borrow().as_ref()
      {
        cb(&enum_name_cb, variant);
      }
    });
    gbox.prepend(&dropdown);
  }

  let mut child = gbox.first_child();
  while let Some(w) = child
  {
    if let Some(child_box) = w.downcast_ref::<gtk4::Box>()
    {
      install_switchers(child_box, registry, on_switch);
    }
    child = w.next_sibling();
  }
}
