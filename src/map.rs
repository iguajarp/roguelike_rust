use rltk::{RandomNumberGenerator, Algorithm2D, Point, BaseMap};

use super::*;
use std::cmp::{max, min};

// Clone override the default move behavior on assignment
// PartialEq allows == . Ex: tile_type == TileType::Wall
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Map {
    /// Get the index from the map. From 0 to 80*50 in a map of that size
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * 80) + x as usize
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80 * 50 {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80 * 50 {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx:usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }
}


/// It defines a map using a Vec<TileType>. So, it doesn't use cartesanian coor.
/// But index from 0 to the area of the map.
/// It places walls around the outer edges of the map, and then adds 400 random
/// walls anywhere that isn't the player's starting point.
pub fn new_map_rooms_and_corridors() -> Map {
    let mut map = Map {
        tiles: vec![TileType::Wall; 80*50],
        rooms: Vec::new(),
        width: 80,
        height: 50,
        revealed_tiles : vec![false; 80*50],
        visible_tiles : vec![false; 80*50],
    };

    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);

        // - w is for not surpassing the width of the map. Same for y
        let x = rng.roll_dice(1, map.width - w - 1);
        let y = rng.roll_dice(1, map.height - h - 1);
        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;

        for other_room in map.rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false
            }
        }
        if ok {
            map.apply_room_to_map(&new_room);

            if !map.rooms.is_empty() {
                let (new_x, new_y) = new_room.center();

                // rooms.len() - 1. Last room added.
                let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();

                // Connect the last room with the new one. Using horizontal and vertical tunnels
                if rng.range(0, 2) == 1 {
                    map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                    map.apply_vertical_tunnel(prev_y, new_y, new_x);
                } else {
                    map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                    map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                }
            }

            map.rooms.push(new_room);
        }
    }

    map
}
