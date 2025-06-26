// Copyright (c) 2025 air (https://yuanair.github.io).
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, version 3 of the License only.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use annasul::app::apps::rust::{HostTriple, Profile, Toolchain};
use gtk4::{
    prelude::*,
    Application,
    ApplicationWindow,
    Button,
    ComboBoxText,
    Entry,
    Label,
    Notebook,
    Orientation,
};
fn build_combo<T: Default + Debug + Display + FromStr + 'static>(
    combo: ComboBoxText, all: &[T], f: fn(&T),
) -> ComboBoxText {
    for i in all.iter() {
        combo.append(Some(&i.to_string()), &i.to_string());
    }
    combo.set_active_id(Some(&T::default().to_string()));
    combo.connect_changed(move |combo| {
        let active_id = combo.active_id().and_then(|id| id.parse().ok());
        if let Some(id) = active_id {
            let selected = T::from(id);
            f(&selected);
        }
    });
    combo
}
fn build_notebook<T: Debug + Display + FromStr + 'static>(
    notebook: Notebook, all: &[T], f: fn(i32) -> bool,
) -> Notebook {
    for i in all.iter() {
        notebook.append_page(
            &Label::new(Some(&i.to_string())),
            Some(&Label::new(Some(&i.to_string()))),
        );
    }
    notebook.set_current_page(Some(0));
    notebook.connect_change_current_page(move |notebook, i| f(i));
    notebook
}
#[tokio::main]
async fn main() -> glib::ExitCode {
    env_logger::init();
    let app = Application::builder().application_id("yuanair.github.io").build();
    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title(env!("CARGO_PKG_NAME"))
            .default_width(800)
            .default_height(600)
            .build();
        window.present();
        let notebook = Notebook::builder().build();
        window.set_child(Some(&notebook));
        for i in 1..=3 {
            let vbox = gtk4::Box::new(Orientation::Vertical, 10);
            vbox.append(&build_combo(
                ComboBoxText::new(),
                &[Profile::Minimal, Profile::Default, Profile::Complete],
                |profile| {
                    println!("profile: {profile}");
                },
            ));
            vbox.append(&build_combo(
                ComboBoxText::new(),
                &[
                    Toolchain::Stable,
                    Toolchain::Beta,
                    Toolchain::Nightly,
                    Toolchain::None,
                ],
                |toolchain| {
                    println!("toolchain: {toolchain}");
                },
            ));
            vbox.append(&build_notebook(
                Notebook::new(),
                &[HostTriple::Host, HostTriple::Target("".to_string())],
                |i| {
                    println!("host: {i}");
                    true
                },
            ));
            let entry = Entry::builder().placeholder_text("Input").margin_top(20).build();
            vbox.append(&entry);
            let button = Button::with_label("Install");
            vbox.append(&button);
            button.connect_clicked(move |_| {
                println!("Input {i}: {}", entry.text());
            });
            notebook.append_page(&vbox, Some(&Label::new(Some(&format!("Preset {i}")))));
        }
    });
    app.run()
}
