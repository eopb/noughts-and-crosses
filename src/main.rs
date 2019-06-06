mod game;

use gtk::{prelude::*, StyleContext};
use shadow_clone::shadow_clone;
use std::{cell::RefCell, rc::Rc};

const GLADE_SRC: &str = include_str!("../ui/ui.glade");
const CSS: &str = include_str!("../ui/style.css");

type ButtonArray = [[LabeledButton; 3]; 3];

#[derive(Clone)]
pub struct LabeledButton {
    button: gtk::Button,
    label: gtk::Label,
}

impl LabeledButton {
    fn get(builder: &gtk::Builder, key: &str) -> Self {
        Self {
            button: builder.get_object(&format!("button-{}", key)).unwrap(),
            label: builder.get_object(&format!("label-{}", key)).unwrap(),
        }
    }
    fn set_label(&self, l: &str) {
        self.label.set_label(l);
    }
    fn get_style_context(&self) -> StyleContext {
        self.button.get_style_context()
    }
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let builder = gtk::Builder::new_from_string(GLADE_SRC);

    let window: gtk::Window = builder.get_object("main-window").unwrap();

    let about_button: gtk::Button = builder.get_object("about-button").unwrap();
    let about_window: gtk::Window = builder.get_object("about-window").unwrap();
    {
        shadow_clone!(about_window);
        about_button.connect_clicked(move |_| {
            about_window.show_all();
        });
    }
    about_window.connect_delete_event(|x, _| Inhibit(x.hide_on_delete()));

    let restart_button: gtk::Button = builder.get_object("restart").unwrap();

    let status: gtk::Label = builder.get_object("status").unwrap();

    let game_state = Rc::new(RefCell::new(game::State::new()));

    let button_array = get_button_array(&builder);

    // window.set_icon_from_file("logo.svg").unwrap();

    for (r_index, row) in button_array.clone().iter().enumerate() {
        for (c_index, button) in row.iter().enumerate() {
            shadow_clone!(game_state, status, button_array, restart_button);
            let label = (*button).label.clone();
            (*button).button.connect_clicked(move |_| {
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
    {
        shadow_clone!(game_state, status);
        restart_button.connect_clicked(move |bself| {
            button_array.iter().flatten().cloned().for_each(|button| {
                button.set_label("");
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
    let get_button_with_label = |x| LabeledButton::get(builder, x);
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
