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

    ///Create a wall
    fn apply_wall_to_map(&mut self, wall: &Rect){
        for y in wall.y1..= wall.y2 {
            for x in wall.x1..= wall.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Wall;
            }
        }
    }
    
    /// Maps a new map with walls
    pub fn new_map_walls() -> Map {
        let mut map = Map{
            tiles : vec![TileType::Floor; 80*50],
            width : 80,
            height: 50,
            walls : Vec::new(),
        };

        const MAX_WALLS : i32 = 15;
        const MIN_SIZE : i32 = 6;
        const MAX_SIZE : i32 = 12;

        let mut rng = RandomNumberGenerator::new();

        //Generate a set of walls
        for _i in 0..MAX_WALLS {

            let mut x = 0;
            let mut y = 0;
            let mut width = 1;
            let mut height = 1;

            //roll to see if wall is horizontal or vertical
            let roll = rng.roll_dice(1,2); 
            match roll {
                1 => 
                { 
                    // horizontal wall
                    width = rng.range(MIN_SIZE, MAX_SIZE);
                    x = rng.roll_dice(1, map.width - width - 1) - 1;
                    y = rng.roll_dice(1, map.height - 1) - 1;
                }
                _ =>
                {
                    // vertical wall
                    height = rng.range(MIN_SIZE, MAX_SIZE);
                    x = rng.roll_dice(1, map.width - 1) - 1;
                    y = rng.roll_dice(1, map.height - height - 1) - 1;
                }
            };

            let new_wall = Rect::new(x,y,width,height);

            let mut position_ok = true;
            for other_wall in map.walls.iter() {
                if new_wall.intersect(other_wall){
                    position_ok = false;
                }
            }


            if position_ok {
                map.apply_wall_to_map(&new_wall);
                map.walls.push(new_wall);
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
        ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}