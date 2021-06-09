use specs::prelude::*;
use specs_derive::*;
use rltk::{RGB};

#[derive(Component, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}


#[derive(Component, Clone, Copy, Debug)]
pub struct Player {
    pub hp: i32,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Monster {
    pub hp: i32,
}


