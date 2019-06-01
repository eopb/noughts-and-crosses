mod game;

use gtk::{prelude::*, Button, Label, Window};
use std::{cell::RefCell, rc::Rc};

const GLADE_SRC: &str = include_str!("../idea.glade");

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let builder = gtk::Builder::new_from_string(GLADE_SRC);

    let window: Window = builder.get_object("main-window").unwrap();

    let status: Label = builder.get_object("status").unwrap();

    let game_state = Rc::new(RefCell::new(game::State::new()));

    let button_array: [[(Button, Label); 3]; 3] = [
        [
            (
                builder.get_object("button-1-1").expect("test"),
                builder.get_object("label-1-1").unwrap(),
            ),
            (
                builder.get_object("button-1-2").unwrap(),
                builder.get_object("label-1-2").unwrap(),
            ),
            (
                builder.get_object("button-1-3").unwrap(),
                builder.get_object("label-1-3").unwrap(),
            ),
        ],
        [
            (
                builder.get_object("button-2-1").unwrap(),
                builder.get_object("label-2-1").unwrap(),
            ),
            (
                builder.get_object("button-2-2").unwrap(),
                builder.get_object("label-2-2").unwrap(),
            ),
            (
                builder.get_object("button-2-3").unwrap(),
                builder.get_object("label-2-3").unwrap(),
            ),
        ],
        [
            (
                builder.get_object("button-3-1").unwrap(),
                builder.get_object("label-3-1").unwrap(),
            ),
            (
                builder.get_object("button-3-2").unwrap(),
                builder.get_object("label-3-2").unwrap(),
            ),
            (
                builder.get_object("button-3-3").unwrap(),
                builder.get_object("label-3-3").unwrap(),
            ),
        ],
    ];
    for (r_index, row) in button_array.iter().enumerate() {
        for (index, button) in row.iter().enumerate() {
            {
                let game_state = game_state.clone();
                let status = status.clone();
                let label = (*button).1.clone();
                (*button).0.connect_clicked(move |_| {
                    game_state
                        .clone()
                        .replace_with(|x| x.next(&label, &status, r_index, index));
                    dbg!(&game_state);
                });
            }
        }
    }

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
