use gtk::{prelude::*, Button, Window};
use std::{cell::RefCell, rc::Rc};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("../idea.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    let window: Window = builder.get_object("main-window").unwrap();

    let game_state = Rc::new(RefCell::new(GameState::new()));

    let button_array: [[Button; 3]; 3] = [
        [
            builder.get_object("button-1-1").unwrap(),
            builder.get_object("button-1-2").unwrap(),
            builder.get_object("button-1-3").unwrap(),
        ],
        [
            builder.get_object("button-2-1").unwrap(),
            builder.get_object("button-2-2").unwrap(),
            builder.get_object("button-2-3").unwrap(),
        ],
        [
            builder.get_object("button-3-1").unwrap(),
            builder.get_object("button-3-2").unwrap(),
            builder.get_object("button-3-3").unwrap(),
        ],
    ];

    for (r_index, row) in button_array.iter().enumerate() {
        for (index, button) in row.iter().enumerate() {
            {
                let game_state = game_state.clone();
                button.connect_clicked(move |button| {
                    game_state
                        .clone()
                        .replace_with(|x| x.place(button, index, r_index));
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

#[derive(Copy, Clone, Debug)]
enum Player {
    X,
    O,
}
use Player::{O, X};

#[derive(Debug, Clone, Copy)]
struct GameState {
    board: [[Option<Player>; 3]; 3],
    current: Player,
}

impl GameState {
    fn new() -> Self {
        Self {
            board: [[None; 3]; 3],
            current: X,
        }
    }
    fn place(mut self, current_button: &Button, x: usize, y: usize) -> Self {
        if self.board[x][y].is_none() {
            current_button.set_label(match self.current {
                X => "x",
                O => "O",
            });
            self.board[x][y] = Some(self.current);
            self.current = match self.current {
                X => O,
                O => X,
            };
            self
        } else {
            println!("Tile already taken.");
            self
        }
    }
}
