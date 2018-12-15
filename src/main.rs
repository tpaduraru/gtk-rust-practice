extern crate gtk;

use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window1 = Window::new(WindowType::Toplevel);
    window1.set_title("First GTK+ Program");
    window1.set_default_size(350, 70);
    let window2 = Window::new(WindowType::Toplevel);
    window2.set_title("First GTK+ Program");
    window2.set_default_size(350, 70);
    
    let button1 = Button::new_with_label("Click me!");
    let button2 = Button::new_with_label("No! Click me!");
    window1.add(&button1);
    window2.add(&button2);
    window1.show_all();
    window2.show_all();

    window1.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    window2.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button1.connect_clicked(|_| {
        println!("Clicked!");
    });
    
    button2.connect_clicked(|_| {
        println!("Ha, Clicked!");
    });


    gtk::main();
}
