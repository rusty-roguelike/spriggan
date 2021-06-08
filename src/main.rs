use rltk::{GameState, Rltk};
use rltk::RGB;
use specs::prelude::*;
mod components;
use components::*;
mod map;
use map::*;
mod rect;
pub use rect::Rect;
mod player;
use player::*;

pub struct State {
    ecs: World
}

pub struct MonsterAi {}

impl<'a> System<'a> for MonsterAi {

    type SystemData = ( ReadStorage<'a, Position>,
                        ReadStorage<'a, Monster>); 

    fn run(&mut self, data: Self::SystemData) {
        let (pos, monster) = data;

        for(pos, _monster) in (&pos, &monster).join() {
            for _x in pos.x-6..pos.x+6 {
                for _y in pos.y-6..pos.y+6 {
                    // look for player in this range and move toward them
                }
            }
        }
    }
}


impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}


impl State {
    fn run_systems(&mut self) {
        let mut mob = MonsterAi{};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("spriggan")
        .build()?;
    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();

    let map : Map = Map::new_map_walls();


    // Spawn player somewhere in top-left 100 tiles
    let (player_x, player_y) = map.get_empty_tile_in_section(0,0);
    
    gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{
            hp: 10
        })
        .build();



    //spawn 10 monsters
    let mut rng = rltk::RandomNumberGenerator::new();
    for _i in 1..= 10{
        
        let glyph : rltk::FontCharType;
        let roll = rng.roll_dice(1,4);

        match roll {
            1 => { glyph = rltk::to_cp437('X') }
            2 => { glyph = rltk::to_cp437('O') }
            3 => { glyph = rltk::to_cp437('*') }
            _ => { glyph = rltk::to_cp437('^') }
        }

        let (section_x, section_y) = (rng.range(1,7), rng.range(1,4));
        let (monster_x, monster_y) = map.get_empty_tile_in_section(section_x, section_y);

        gs.ecs.create_entity()
            .with(Position{ x:monster_x, y:monster_y })
            .with(Renderable{
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Monster{
                hp: 2
            })
            .build();
    }       

    gs.ecs.insert(map);

    rltk::main_loop(context, gs)
}
