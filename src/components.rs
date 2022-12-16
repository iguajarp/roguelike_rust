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
struct Position {
    // Just a POD, plain old data, is common for pure ECS. No logic
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}