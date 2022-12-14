use rltk::{GameState, Rltk, VirtualKeyCode, RGB}; // extern is an old keyword, not needed now
use specs::prelude::*; // macro_use is an old keyword too. Not needed anymore
use specs_derive::Component;
use std::cmp::{max, min};

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

struct State {
    ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        // You need a Position to know where to draw, and Renderable to know what to draw!
        // Join return only entities that have both, like a db INNER JOIN. Works with tuples
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .build(); // .build() takes the assembled entity and does the hard part - actually putting together all of the disparate parts into the right parts of the ECS for you.

        // The 0..10  is a range - and offers an iterator for Rust to navigate
        for i in 0..10 {
            gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
        }

    rltk::main_loop(context, gs)
}
