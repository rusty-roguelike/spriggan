use rltk::{GameState, Rltk, RGB, VirtualKeyCode, console};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;
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

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {

    type SystemData = ( ReadStorage<'a, Position>,
                        ReadStorage<'a, Monster>); 

    fn run(&mut self, data: Self::SystemData) {
        let (pos, monster) = data;

        for(pos, _monster) in (&pos, &monster).join() {
            console::log("Monster AI placeholder");
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
        let mut mob = MonsterAI{};
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



    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Monster{})
        .build();

        let map : Map = Map::new_map_walls();
        gs.ecs.insert(map);


    //spawn 10 monsters
    let mut rng = rltk::RandomNumberGenerator::new();
    for i in 1..=10 {
        let (x,y) = (rng.range(0,81), rng.range(0,46)); 
        
        let glyph : rltk::FontCharType;
        let roll = rng.roll_dice(1,4);

        match roll {
            1 => { glyph = rltk::to_cp437('X') }
            2 => { glyph = rltk::to_cp437('O') }
            3 => { glyph = rltk::to_cp437('*') }
            _ => { glyph = rltk::to_cp437('^') }
        }
        

        gs.ecs.create_entity()
            .with(Position{ x, y })
            .with(Renderable{
                glyph: glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .build();

    }       


    rltk::main_loop(context, gs)
}