use gtk::prelude::*;
use gtk::{glib, Application};
mod ui;
mod config;

const APP_ID: &str = "com.kengachu.Logout";

fn start_app() -> glib::ExitCode {
    gtk::init().unwrap();
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(ui::build_ui);

    app.run()
}

fn main() -> glib::ExitCode {
    start_app()
}
