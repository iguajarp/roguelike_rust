use crate::Player;

use super::{Map, Position, Viewshed};
use rltk::{field_of_view, Point};
use specs::{prelude::*, storage::GenericReadStorage};

pub struct VisibilitySystem {}

/// A system does something independently
impl<'a> System<'a> for VisibilitySystem {
    // those are the storages that the system will use
    type SystemData = (
        ReadStorage<'a, Player>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        WriteExpect<'a, Map>, // We used ReadExpect, because not having a map is a failure.
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player, entities, mut viewshed, pos, mut map) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            viewshed.visible_tiles.clear();

            // Checks the visibles tiles and return a Vec with the Point(x, y) of the visible tiles
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed
                .visible_tiles
                .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

            let p: Option<&Player> = player.get(ent);
            if let Some(_p) = p {
                for vis in viewshed.visible_tiles.iter() {
                    let idx = map.xy_idx(vis.x, vis.y);
                    map.revealed_tiles[idx] = true;
                }
            }
        }
    }
}
