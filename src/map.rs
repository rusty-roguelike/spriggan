use rltk::{ RGB, Rltk, RandomNumberGenerator, BaseMap, Algorithm2D, Point };
use super::{Rect};
use std::cmp::{max, min};
use specs::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

#[derive(Default)]
pub struct Map {
    pub tiles : Vec<TileType>,
    pub width : i32,
    pub height : i32,
    pub walls : Vec<Rect>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    ///Create a horizontal wall
    fn apply_horizontal_wall(&mut self, x1:i32, x2:i32, y:i32){
        for x in min(x1,x2) ..=max(x1,x2) {
            let idx = self.xy_idx(x,y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Wall;
            }
        }
    }

    ///Create a vertical wall
    fn apply_vertical_wall(&mut self, y1:i32, y2:i32, x:i32){
        for y in min(y1,y2) ..=max(y1,y2) {
            let idx = self.xy_idx(x,y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Wall;
            }
        }
    }
    
    /// Maps a new map with walls
    pub fn new_map_walls() -> Map {
        let mut map = Map{
            tiles : vec![TileType::Floor; 80*50],
            width : 80,
            height: 50,
        };

        const MAX_WALLS : i32 = 15;
        const MIN_SIZE : i32 = 8;
        const MAX_SIZE : i32 = 16;

        let mut rng = RandomNumberGenerator::new();

        //Generate a set of walls
        while self.walls.len() < MAX_WALLS {
            let wall_len = rng.range(MIN_SIZE, MAX_SIZE);

            //roll to see if wall is horizontal or vertical
            let 
            let x = rng.roll_dice(1, map.width - wall_len - 1) - 1;
            let y = rng.roll_dice(1, map.height - wall_len - 1) - 1;
            let new_wall = Rect::new(x, y, len);
            let mut ok = true;

            for other_room in map.walls.iter() {
                if new_room.intersect(other_room) { ok = false }
            }
            if ok {
                map.apply_room_to_map(&new_room);

/*                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len()-1].center();
                    if rng.range(0,2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }*/

                map.walls.push(new_room);
            }
        }

        map
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx:usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

pub fn draw_map(ecs: &World, ctx : &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;
    for (idx,tile) in map.tiles.iter().enumerate() {
        // Render a tile depending upon the tile type

        if map.revealed_tiles[idx] {
            let glyph;
            let mut fg;
            match tile {
                TileType::Floor => {
                    glyph = rltk::to_cp437('.');
                    fg = RGB::from_f32(0.0, 0.5, 0.5);
                }
                TileType::Wall => {
                    glyph = rltk::to_cp437('#');
                    fg = RGB::from_f32(0., 1.0, 0.);
                }
            }
            if !map.visible_tiles[idx] { fg = fg.to_greyscale() }
            ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
