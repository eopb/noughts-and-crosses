use gtk::{prelude::*, Label};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    X,
    O,
}
use Player::{O, X};

impl Player {
    fn show(self) -> &'static str {
        match self {
            X => "X",
            O => "O",
        }
    }
    fn swap(self) -> Self {
        match self {
            X => O,
            O => X,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GameState {
    board: [[Option<Player>; 3]; 3],
    current: Player,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: [[None; 3]; 3],
            current: X,
        }
    }
    pub fn next(
        mut self,
        current_button: &Label,
        status: &Label,
        row: usize,
        column: usize,
    ) -> Self {
        dbg!(&self.winner());
        if self.board[row][column].is_none() {
            current_button.set_label(self.current.show());
            self.board[row][column] = Some(self.current);
            self.current = self.current.swap();
            status.set_markup(&format!("Player {} turn", self.current.show()));
            self
        } else {
            status.set_markup("Tile already taken.");
            self
        }
    }
    pub fn winner(&self) -> Option<Player> {
        for possible_winner in [O, X].iter() {
            for index in 0..3 {
                if self.board[index].iter().all(|x| match x {
                    Some(x) => x == possible_winner,
                    None => false,
                }) {
                    return Some(*possible_winner);
                };
            }
        }
        None
    }
}
