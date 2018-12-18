extern crate gtk;

use gtk::prelude::*;
use gtk::{Button, Window, WindowType, Label};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(800, 600);
    
    let mut label = Label::new("Hello");
    let button2 = Button::new_with_label("Click me!");
    
    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    v_box.pack_start(&label, true, true, 3);
    v_box.pack_start(&button2, false, false, 3);
    window.add(&v_box);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });


    //button1.connect_clicked(|_| {
        //println!("Clicked!");
    //});
    
    button2.connect_clicked(|_| {
        println!("Ha, Clicked!");
    });


    gtk::main();
}
