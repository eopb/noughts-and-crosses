use gtk::{prelude::*, Button, Label};

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
pub struct State {
    board: [[Option<Player>; 3]; 3],
    current: Player,
}

impl State {
    pub fn new() -> Self {
        Self {
            board: [[None; 3]; 3],
            current: X,
        }
    }
    pub fn next(
        mut self,
        current_button: &Label,
        all_buttons: &[[(Button, Label); 3]; 3],
        status: &Label,
        row: usize,
        column: usize,
    ) -> Self {
        dbg!(&self.winner(all_buttons));
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
    pub fn winner(&self, all_buttons: &[[(Button, Label); 3]; 3]) -> Option<Player> {
        self.check([(0, 0), (1, 0), (2, 0)], all_buttons)
    }
    fn check(
        &self,
        checks: [(usize, usize); 3],
        all_buttons: &[[(Button, Label); 3]; 3],
    ) -> Option<Player> {
        for possible_winner in &[O, X] {
            if checks
                .iter()
                .map(|(x, y)| self.board[*x][*y])
                .all(|x| x == Some(*possible_winner))
            {
                for check in checks.iter() {
                    all_buttons[check.0][check.1].0.get_style_context().add_class("won");
                }
                return Some(*possible_winner);
            }
        }
        None
    }
}
