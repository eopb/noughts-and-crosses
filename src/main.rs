mod game;

use gtk::{prelude::*};
use std::{cell::RefCell, rc::Rc};

const GLADE_SRC: &str = include_str!("../ui/ui.glade");
const CSS: &str = include_str!("../ui/style.css");

type ButtonArray = [[(gtk::Button, gtk::Label); 3]; 3];

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let builder = gtk::Builder::new_from_string(GLADE_SRC);

    let window: gtk::Window = builder.get_object("main-window").unwrap();
    let restart_button: gtk::Button = builder.get_object("restart").unwrap();

    let status: gtk::Label = builder.get_object("status").unwrap();

    let game_state = Rc::new(RefCell::new(game::State::new()));

    let button_array = get_button_array(&builder);

    for (r_index, row) in button_array.clone().iter().enumerate() {
        for (c_index, button) in row.iter().enumerate() {
            {
                let game_state = game_state.clone();
                let status = status.clone();
                let label = (*button).1.clone();
                let button_array = button_array.clone();
                let restart_button = restart_button.clone();
                (*button).0.connect_clicked(move |_| {
                    game_state.clone().replace_with(|x| {
                        x.next(
                            &label,
                            &button_array,
                            &status,
                            &restart_button,
                            r_index,
                            c_index,
                        )
                    });
                });
            }
        }
    }
    {
        let game_state = game_state.clone();
        let status = status.clone();
        restart_button.connect_clicked(move |bself| {
            button_array
                .iter()
                .flatten()
                .cloned()
                .for_each(|(button, label)| {
                    label.set_label("");
                    button.get_style_context().remove_class("won")
                });
            game_state.replace_with(|_| game::State::new());
            bself.get_style_context().remove_class("should-restart");
            status.set_label("Game on");
        });
    }

    apply_css(&window);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

fn get_button_array(builder: &gtk::Builder) -> ButtonArray {
    let get_button_with_label = |x| {
        (
            builder.get_object(&format!("button-{}", x)).unwrap(),
            builder.get_object(&format!("label-{}", x)).unwrap(),
        )
    };
    [
        [
            get_button_with_label("1-1"),
            get_button_with_label("1-2"),
            get_button_with_label("1-3"),
        ],
        [
            get_button_with_label("2-1"),
            get_button_with_label("2-2"),
            get_button_with_label("2-3"),
        ],
        [
            get_button_with_label("3-1"),
            get_button_with_label("3-2"),
            get_button_with_label("3-3"),
        ],
    ]
}

fn apply_css(window: &gtk::Window) {
    let screen = window.get_screen().unwrap();
    let style = gtk::CssProvider::new();
    let _ = gtk::CssProviderExt::load_from_data(&style, CSS.as_bytes());
    gtk::StyleContext::add_provider_for_screen(&screen, &style, gtk::STYLE_PROVIDER_PRIORITY_USER);
}
