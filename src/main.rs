use rltk::RGB;
use rltk::{GameState, Rltk};
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
    world: World,
    run_state: RunState,
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

pub struct MonsterAi {}

impl<'a> System<'a> for MonsterAi {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut pos_storage, mon_storage, mut player_storage) = data;

        let mut player_positions: Vec<Position> = vec![];
        for (_player, player_pos) in (&player_storage, &pos_storage).join() {
            player_positions.push(*player_pos);
        }

        for (_mon, mon_pos) in (&mon_storage, &mut pos_storage).join() {
            for player_pos in player_positions.iter() {
                if adjacent_positions(10, *player_pos, *mon_pos) {
                    let new_pos = move_toward(&mon_pos, &player_pos);
                    mon_pos.x = new_pos.x;
                    mon_pos.y = new_pos.y;
                } else {
                    let mut rng = rltk::RandomNumberGenerator::new();
                    let roll = rng.roll_dice(1, 4);

                    match roll {
                        1 => {
                            mon_pos.x += 1;
                        }
                        2 => {
                            mon_pos.x -= 1;
                        }
                        3 => {
                            mon_pos.y += 1;
                        }
                        _ => {
                            mon_pos.y -= 1;
                        }
                    }
                }
            }
        }

        for (player_pos, player) in (&pos_storage, &mut player_storage).join() {
            for (mon_pos, _monster) in (&pos_storage, &mon_storage).join() {
                if adjacent_positions(1, *player_pos, *mon_pos) {
                    player.hp -= 1;
                    println!("OUCH! Your HP is , {:?}", &player.hp);
                }
            }
        }
    }
}

fn move_toward(first: &Position, second: &Position) -> Position {
    let delta_x = (second.x - first.x).abs();
    let delta_y = (second.y - first.y).abs();
    if delta_x < delta_y {
        if first.x < second.x {
            Position {
                x: { first.x + 1 },
                y: first.y,
            }
        } else {
            Position {
                x: { first.x - 1 },
                y: first.y,
            }
        }
    } else if first.y < second.y {
        Position {
            x: first.x,
            y: { first.y + 1 },
        }
    } else {
        Position {
            x: first.x,
            y: { first.y - 1 },
        }
    }
}

pub struct PlayerCombat {}

impl<'a> System<'a> for PlayerCombat {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_pos, monster_pos, monster, mut player) = data;

        let mut monster_positions: Vec<Position> = vec![];
        for (&pos, _mon) in (&monster_pos, &monster).join() {
            monster_positions.push(pos);
        }

        for (&pos, player) in (&player_pos, &mut player).join() {
            for mon_pos in monster_positions.iter() {
                if (mon_pos.x == pos.x) && (mon_pos.y == pos.y) {
                    player.hp -= 1;
                    println!("player hp is , {:?}", &player.hp);
                }
            }
        }
    }
}

pub fn adjacent_positions(within: i32, pos_1: Position, pos_2: Position) -> bool {
    if (pos_1.x - pos_2.x).abs() <= within && (pos_1.y - pos_2.y).abs() <= within {
        return true;
    }
    false
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if self.run_state == RunState::Running {
            self.run_systems();
            self.run_state = RunState::Paused;
        } else {
            self.run_state = player_input(self, ctx);
        }
        clean_up_dead(&mut self.world);
        draw_map(&self.world, ctx);

        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
        ctx.print_color(
            17,
            49,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            "Arrows to move - Space to attack around you",
        );
    }
}

fn clean_up_dead(ecs: &mut World) {
    let mut dead_entities: Vec<Entity> = vec![];
    {
        let entities = ecs.entities();
        let monsters = ecs.read_storage::<Monster>();

        for (monster, entity) in (&monsters, &entities).join() {
            if monster.hp <= 0 {
                dead_entities.push(entity);
            }
        }
    }
    for entity in dead_entities {
        ecs.delete_entity(entity).unwrap();
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut mob = MonsterAi {};
        mob.run_now(&self.world);
        self.world.maintain();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("spriggan").build()?;
    let mut game_state = State {
        world: World::new(),
        run_state: RunState::Running,
    };
    game_state.world.register::<Position>();
    game_state.world.register::<Renderable>();
    game_state.world.register::<Player>();
    game_state.world.register::<Monster>();

    let map: Map = Map::new_map_walls();

    // Spawn player somewhere in top-left 100 tiles
    let (player_x, player_y) = map.get_empty_tile_in_section(0, 0);

    game_state
        .world
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player { hp: 10 })
        .build();

    //spawn 10 monsters
    let mut rng = rltk::RandomNumberGenerator::new();
    for _i in 1..=10 {
        let glyph: rltk::FontCharType;
        let roll = rng.roll_dice(1, 4);

        match roll {
            1 => glyph = rltk::to_cp437('X'),
            2 => glyph = rltk::to_cp437('O'),
            3 => glyph = rltk::to_cp437('*'),
            _ => glyph = rltk::to_cp437('^'),
        }

        let (section_x, section_y) = (rng.range(1, 7), rng.range(1, 4));
        let (monster_x, monster_y) = map.get_empty_tile_in_section(section_x, section_y);

        game_state
            .world
            .create_entity()
            .with(Position {
                x: monster_x,
                y: monster_y,
            })
            .with(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Monster { hp: 2 })
            .build();
    }

    game_state.world.insert(map);

    rltk::main_loop(context, game_state)
}

#[test]
fn test_adjacent_positions() {
    assert!(adjacent_positions(
        1,
        Position { x: 1, y: 1 },
        Position { x: 1, y: 1 }
    ));
    assert!(adjacent_positions(
        1,
        Position { x: 1, y: 1 },
        Position { x: 2, y: 2 }
    )); // diagonals work!
    assert!(!adjacent_positions(
        1,
        Position { x: 2, y: 8 },
        Position { x: 1, y: 1 }
    ))
}
