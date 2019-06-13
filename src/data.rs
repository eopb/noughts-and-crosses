use gtk::{prelude::*, StyleContext};
use shadow_clone::shadow_clone;
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct LabeledButton {
    pub button: gtk::Button,
    pub label: gtk::Label,
}

impl LabeledButton {
    pub fn get(builder: &gtk::Builder, key: &str) -> Self {
        Self {
            button: builder.get_object(&format!("button-{}", key)).unwrap(),
            label: builder.get_object(&format!("label-{}", key)).unwrap(),
        }
    }
    pub fn set_label(&self, l: &str) {
        self.label.set_label(l);
    }
    pub fn get_style_context(&self) -> StyleContext {
        self.button.get_style_context()
    }
    pub fn connect_clicked<F: Fn(&gtk::Button) + 'static>(&self, f: F) {
        self.button.connect_clicked(f);
    }
}

#[derive(Clone)]
pub struct Board(pub [[Tile; 3]; 3]);

impl Board {
    pub fn get(builder: &gtk::Builder) -> Self {
        let get_button = |x| Tile::get(builder, x);
        let board = Self([
            [get_button("1-1"), get_button("1-2"), get_button("1-3")],
            [get_button("2-1"), get_button("2-2"), get_button("2-3")],
            [get_button("3-1"), get_button("3-2"), get_button("3-3")],
        ]);
        board
    }
    pub fn clear(&mut self) {
        
    }
}

#[derive(Clone)]
pub struct Tile {
    button: LabeledButton,
    status: Option<Player>,
}

impl Tile {
    pub fn get(builder: &gtk::Builder, key: &str) -> Self {
        Tile {
            button: LabeledButton::get(builder, key),
            status: None,
        }
    }
    pub fn set(&mut self, p: Player) {
        self.button.label.set_label(p.show());
        self.status = Some(p);
    }
    pub fn clear(&mut self) {
        self.button.label.set_label("");
        self.status = None;
    }
    pub fn get_style_context(&self) -> StyleContext {
        self.button.get_style_context()
    }
    pub fn connect_clicked<F: Fn(&gtk::Button) + 'static>(&self, f: F) {
        self.button.connect_clicked(f);
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    X,
    O,
}
use Player::{O, X};

impl Player {
    pub fn show(self) -> &'static str {
        match self {
            X => "X",
            O => "O",
        }
    }
    pub fn swap(self) -> Self {
        match self {
            X => O,
            O => X,
        }
    }
}
