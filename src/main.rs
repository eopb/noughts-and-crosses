use gtk::prelude::*;
use gtk::{Button, Grid, Window, WindowType};
use std::cell::Cell;
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Noughts and crosses GTK");
    window.set_default_size(350, 350);

    let mut game_state = Rc::new(RefCell::new(GameState::new()));

    let button_array = [
        [
            Button::new_with_label(""),
            Button::new_with_label(""),
            Button::new_with_label(""),
        ],
        [
            Button::new_with_label(""),
            Button::new_with_label(""),
            Button::new_with_label(""),
        ],
        [
            Button::new_with_label(""),
            Button::new_with_label(""),
            Button::new_with_label(""),
        ],
    ];

    let grid = Grid::new();
    grid.insert_column(3);
    grid.insert_row(3);
    grid.set_column_homogeneous(true);
    grid.set_row_homogeneous(true);

    for (r_index, row) in button_array.iter().enumerate().map(|(x, y)| ((x), y)) {
        for (index, button) in row.iter().enumerate().map(|(x, y)| ((x), y)) {
            {
                let game_state = game_state.clone();
                button.connect_clicked(move |_| {
                    dbg!(Rc::strong_count(&mut game_state.clone()));
                    game_state.clone().replace_with(|x| x.place(r_index, index));
                    dbg!(&game_state);
                    println!("Clicked!");
                });
            }
            grid.attach(
                button,
                (r_index + 1).try_into().unwrap(),
                (index + 1).try_into().unwrap(),
                1,
                1,
            );
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
    fn test() -> Self {
        Self {
            board: [[Some(O); 3]; 3],
            current: X,
        }
    }
    fn place(mut self, x: usize, y: usize) -> Self {
        self.board[x][y] = Some(self.current);
        self
    }
}
