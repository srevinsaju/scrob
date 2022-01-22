


use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Orientation};


pub fn main_gui() {
    let application = Application::builder()
    .application_id("in.srev.lyrix.desktop")
    .build();
    application.connect_activate(build_ui);

    application.run_with_args(&[""]);
}

fn build_ui(app: &Application) {
    
    
    

    let button = gtk::Button::builder()
    .label("Press me!")
    .margin_top(12)
    .margin_start(12)
    .margin_end(12)
    .margin_bottom(12)
    .build();

    let label = gtk::Label::builder()
    .label("Lyrix")
    .margin_top(12)
    .margin_start(12)
    .margin_end(12)
    .margin_bottom(12)
    .build();

    button.connect_clicked(move |button| {
        println!("Hello World!");
    });


    let gtk_box = gtk::Box::new(Orientation::Vertical, 0);
    gtk_box.append(&label);
    gtk_box.append(&button);
    
    let window = ApplicationWindow::builder()
    .application(app)
    .title("Lyrix Desktop")
    .child(&gtk_box)
    .build();

    window.set_default_size(350, 700);

    // Add action "quit" to `window` taking no parameter

    window.present();
}