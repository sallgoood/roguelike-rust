use tcod::colors::*;
use crate::Rect;
use std::cmp;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;

pub const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };

pub const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

pub type Map = Vec<Vec<Tile>>;

/**
There’s a ton of different ways to create the map. One common alternative is one continuous Vec with MAP_HEIGHT * MAP_WIDTH items. To access a tile on (x, y), you would do map[y * MAP_WIDTH + x]. The advantage is that you only do one array lookup instead of two and iterating over every object in the map is faster because they’re all in the same region of memory.
Or you could treat walls and everything else in the map as just another Object and store them there. This would make the game structure simpler (everything is an Object) and more flexible (just add HP to make a wall destructible, or damage to one that’s supposed to be covered with spikes).
**/
pub fn make_map() -> Map {
    //TODO map_filled_with_unblocked_tiles
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    // create two rooms
    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(50, 15, 10, 15);
    create_room(room1, &mut map);
    create_room(room2, &mut map);
    create_h_tunnel(25, 55, 23, &mut map);
    map
}

fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    // horizontal tunnel. `min()` and `max()` are used in case `x1 > x2`
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    // vertical tunnel
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}
