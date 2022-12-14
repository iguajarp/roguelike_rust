use rltk::{GameState, Rltk, RGB, VirtualKeyCode}; // extern is an old keyword, not needed now
use specs::prelude::*; // macro_use is an old keyword too. Not needed anymore
use std::cmp::{max, min};
use specs_derive::Component;

#[derive(Component)] // The #[macro_use] use specs_derive::Component; earliers versions
struct Position { // Just a POD, plain old data, is common for pure ECS. No logic
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World");
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let gs = State{};
    rltk::main_loop(context, gs)
}