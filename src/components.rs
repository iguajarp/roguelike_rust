use rltk::RGB;
use specs_derive::*;
use specs::prelude::*;


#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component)] // The #[macro_use] use specs_derive::Component; earliers versions
pub struct Position {
    // Just a POD, plain old data, is common for pure ECS. No logic
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct Name {
    pub name : String
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component, Debug)]
pub struct BlocksTile {}