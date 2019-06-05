mod game;

use gtk::{prelude::*, Button, Label, Window};
use std::{cell::RefCell, rc::Rc};

const GLADE_SRC: &str = include_str!("../ui/ui.glade");

const CSS: &str = include_str!("../ui/style.css");

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let builder = gtk::Builder::new_from_string(GLADE_SRC);

    let window: Window = builder.get_object("main-window").unwrap();
    let restart_button: Button = builder.get_object("restart").unwrap();

    let status: Label = builder.get_object("status").unwrap();

    let game_state = Rc::new(RefCell::new(game::State::new()));

    let button_array: [[(Button, Label); 3]; 3] = [
        [
            (
                builder.get_object("button-1-1").unwrap(),
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
    for (r_index, row) in button_array.clone().iter().enumerate() {
        for (index, button) in row.iter().enumerate() {
            {
                let game_state = game_state.clone();
                let status = status.clone();
                let label = (*button).1.clone();
                let button_array = button_array.clone();
                (*button).0.connect_clicked(move |_| {
                    game_state
                        .clone()
                        .replace_with(|x| x.next(&label, &button_array, &status, r_index, index));
                    dbg!(&game_state);
                });
            }
        }
    }
    {
        let game_state = game_state.clone();
        let status = status.clone();
        restart_button.connect_clicked(move |_| {
            button_array
                .iter()
                .flatten()
                .map(|x| x.clone())
                .for_each(|(button,label)| {
                    label.set_label("");
                    button.get_style_context().remove_class("won")
                });
            game_state.replace_with(|_| game::State::new());
            status.set_label("Game on");
        });
    }
    let screen = window.get_screen().unwrap();
    let style = gtk::CssProvider::new();
    let _ = gtk::CssProviderExt::load_from_data(&style, CSS.as_bytes());
    dbg!(CSS);
    gtk::StyleContext::add_provider_for_screen(&screen, &style, gtk::STYLE_PROVIDER_PRIORITY_USER);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
