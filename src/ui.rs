use std::process::Command;

use gtk::{prelude::*, Align};
use gtk::{Application, ApplicationWindow, Button};

use crate::config::load_button_config;
use crate::config::ButtonConfig;

pub fn build_ui(app: &Application) {
    let buttons = load_button_config();
    let button_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .valign(Align::Center)
        .halign(Align::Center)
        .build();

    // let x: Vec<Button> = buttons
    //     .into_iter()
    //     .map(|c| {
    //         let button = Button::builder().label(c.text).build();
    //
    //         button.connect_clicked(move |button| {
    //             Command::new(c.operation.exec.clone())
    //                 .args(c.operation.args.clone())
    //                 .spawn()
    //                 .expect("Could not execute configured command");
    //         });
    //
    //         button
    //     })
    //     .collect();

    for c in buttons.into_iter() {
        let button = Button::builder().label(c.text).build();

        button.connect_clicked(move |button| {
            Command::new(c.operation.exec.clone())
                .args(c.operation.args.clone())
                .spawn()
                .expect("Could not execute configured command");
        });
        button_box.append(&button);
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .resizable(false)
        .child(&button_box)
        .build();

    window.present();
}
