use gtk::prelude::*;
use gtk::{Button, Grid, Window, WindowType};
use std::convert::TryInto;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Noughts and crosses GTK");
    window.set_default_size(350, 350);
    let button_array = dbg!([
        [
            Button::new_with_label("1"),
            Button::new_with_label(""),
            Button::new_with_label("2"),
        ],
        [
            Button::new_with_label(""),
            Button::new_with_label("3"),
            Button::new_with_label(""),
        ],
        [
            Button::new_with_label("4"),
            Button::new_with_label(""),
            Button::new_with_label("5"),
        ],
    ]);

    let grid = Grid::new();
    grid.insert_column(3);
    grid.insert_row(3);
    grid.set_column_homogeneous(true);
    grid.set_row_homogeneous(true);

    for (r_index, row) in button_array
        .iter()
        .enumerate()
        .map(|(x, y)| ((x + 1).try_into().unwrap(), y))
    {
        for (index, button) in row
            .iter()
            .enumerate()
            .map(|(x, y)| ((x + 1).try_into().unwrap(), y))
        {
            grid.attach(button, r_index, index, 1, 1);
            button.connect_clicked(|button| {
                button.set_label("x");
                println!("Clicked!");
            });
        }
    }

    window.add(&grid);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
