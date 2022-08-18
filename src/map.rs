use bracket_lib::prelude::*;
use serde::{Serialize, Deserialize};
use std::cmp::{max, min};

use super::{Rect};

pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 40;

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Floor,
    Wall,
    Air,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub depth: i32,
}

impl Map {
    /// This is simple: it multiplies the y position by the map width (80), and adds x.
    /// This guarantees one tile per location, and efficiently maps it in memory for left-to-right reading.
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn new_map_rooms_and_corridors(new_depth: i32) -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; MAP_WIDTH * MAP_HEIGHT],
            rooms: Vec::new(),
            width: MAP_WIDTH as i32,
            height: MAP_HEIGHT as i32,
            depth: new_depth,
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, MAP_WIDTH as i32 - w - 1) - 1;
            let y = rng.roll_dice(1, MAP_HEIGHT as i32 - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);

            let mut rooms_intersect = false;
            for other_room in map.rooms.iter() {
                if new_room.intersects(other_room) {
                    rooms_intersect = true;
                    break;
                }
            }

            if !rooms_intersect {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_corridor(prev_x, new_x, prev_y);
                        map.apply_vertical_corridor(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_corridor(prev_y, new_y, prev_x);
                        map.apply_horizontal_corridor(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }

    pub fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    pub fn apply_horizontal_corridor(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < MAP_WIDTH * MAP_HEIGHT {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    pub fn apply_vertical_corridor(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < MAP_WIDTH * MAP_HEIGHT {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
}

