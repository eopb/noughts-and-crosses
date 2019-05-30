use gtk::prelude::*;
use gtk::{Button, Grid, Window, WindowType};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let button_size = 1;
    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);
    let button = Button::new_with_label("Click me!");
    button.set_property_height_request(100);
    let button1 = Button::new_with_label("Click me1!");
    button1.set_property_height_request(100);
    let button2 = Button::new_with_label("Click me2!");
    button2.set_property_height_request(100);
    let button3 = Button::new_with_label("Click me3!");
    button3.set_property_height_request(100);
    let button4 = Button::new_with_label("Click me4!");
    button4.set_property_height_request(100);
    let button5 = Button::new_with_label("Click me5!");
    button5.set_property_height_request(100);
    let button6 = Button::new_with_label("Click me6!");
    button6.set_property_height_request(100);
    let button7 = Button::new_with_label("Click me7!");
    button7.set_property_height_request(100);
    let button8 = Button::new_with_label("Click me8!");
    button8.set_property_height_request(100);
    let grid = Grid::new();
    grid.insert_column(3);
    grid.insert_row(3);
    grid.attach(&button, 1, 1, 1, 1);
    grid.attach(&button1, 1, 2, 1, 1);
    grid.attach(&button2, 1, 3, 1, 1);
    grid.attach(&button3, 2, 1, 1, 1);
    grid.attach(&button4, 2, 2, 1, 1);
    grid.attach(&button5, 2, 3, 1, 1);
    grid.attach(&button6, 3, 1, 1, 1);
    grid.attach(&button7, 3, 2, 1, 1);
    grid.attach(&button8, 3, 3, 1, 1);
    window.add(&grid);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        println!("Clicked!");
    });

    gtk::main();
}
