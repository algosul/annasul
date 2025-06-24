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
use gtk4::{Application, ApplicationWindow, Button, Entry, Orientation, prelude::*};
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
        let vbox = gtk4::Box::new(Orientation::Vertical, 10);
        window.set_child(Some(&vbox));
        let entry = Entry::builder().placeholder_text("Input").margin_top(20).build();
        vbox.append(&entry);
        let button = Button::with_label("Ok");
        vbox.append(&button);
        button.connect_clicked(move |_| {
            println!("Input: {}", entry.text());
        });
    });
    app.run()
}
