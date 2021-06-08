use rltk::{ RGB, Rltk, RandomNumberGenerator, BaseMap, Algorithm2D, Point };
use super::{Rect};
use specs::prelude::*;

#[derive(PartialEq, Copy, Clone, Debug)]
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
    ///Given x and y coordinates, return tile index
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    ///Given tile index, return x and y coordinates
    pub fn idx_xy(&self, idx: i32) -> (i32,i32) {
        (idx % 80, idx / 80)
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
    
    /// Maps a new map with randomly placed walls
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

            let x: i32;
            let y: i32;
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

    ///Get an empty (Floor) tile in a section of the map
    ///A section is a 10x10 tile piece of the map, with section (0,0) being the top-left 100 tiles, and section (7,4) being the bottom right 100 tiles
    ///Returns x,y coordinate of empty tile for player/monster placement
    pub fn get_empty_tile_in_section(&self, section_x:i32, section_y:i32) -> (i32,i32)
    {
        let mut section_idxs : Vec<i32> = vec!();

        for x_offset in 0..10 {
            for y_offset in 0..10 {
                section_idxs.push(self.xy_idx(section_x * 10 + x_offset, section_y * 10 + y_offset) as i32);
            }
        }

        
        let mut rng = RandomNumberGenerator::new();
        let mut roll = rng.range(0,99); 

        while self.tiles[section_idxs[roll as usize] as usize] != TileType::Floor {
            roll = rng.range(0,99); 
        }

        self.idx_xy(section_idxs[roll as usize])
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
    for (_idx,tile) in map.tiles.iter().enumerate() {
        // Render a tile depending upon the tile type

        let glyph;
        let fg;
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

#[test]
pub fn test_idx_xy(){
    let map = Map::new_map_walls();
    assert_eq!(map.idx_xy(81), (1,1));
    assert_eq!(map.idx_xy(364), (44,4)); 
}

#[test]
pub fn test_get_empty_tile_in_section() {
    let map = Map::new_map_walls();

    let (x,y) = map.get_empty_tile_in_section(0,0);

    assert!(x >= 0 && x <= 9);
    assert!(y >= 0 && y <= 9);
    assert_eq!(map.tiles[map.xy_idx(x,y) as usize], TileType::Floor);

    let (x,y) = map.get_empty_tile_in_section(3,4);
    assert!(x >= 30 && x <= 39);
    assert!(y >= 0 && y <= 49);
    assert_eq!(map.tiles[map.xy_idx(x,y) as usize], TileType::Floor);
}

#[test]
pub fn test_xy_idx(){
    let map = Map::new_map_walls();
    assert_eq!(map.xy_idx(1,1), 81);
    assert_eq!(map.xy_idx(44,4), 364); 
}
