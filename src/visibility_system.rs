use super::{Map, Position, Viewshed};
use rltk::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem {}

/// A system does something independently
impl<'a> System<'a> for VisibilitySystem {
    // those are the storages that the system will use
    type SystemData = (
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadExpect<'a, Map>, // We used ReadExpect, because not having a map is a failure.
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut viewshed, pos, map) = data;
        for (viewshed, pos) in (&mut viewshed, &pos).join() {
            viewshed.visible_tiles.clear();

            // Checks the visibles tiles and return a Vec with the Point(x, y) of the visible tiles
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed
                .visible_tiles
                .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
        }
    }
}
