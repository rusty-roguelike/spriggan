use super::{Map, Player, Position, State, TileType};
use crate::components::Monster;
use crate::{adjacent_positions, RunState};
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

pub fn try_attack(ecs: &mut World) {
    let player_positions = ecs.read_storage::<Position>();
    let monster_positions = ecs.read_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut monsters = ecs.write_storage::<Monster>();

    for (_player, &p_pos) in (&mut players, &player_positions).join() {
        for (monster, &m_pos) in (&mut monsters, &monster_positions).join() {
            if adjacent_positions(2, p_pos, m_pos) {
                monster.hp -= 1;
                println!("monster hp is , {:?}", &monster.hp);
            }
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        None => return RunState::Paused, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.world),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.world),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.world),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.world),
            VirtualKeyCode::Space => try_attack(&mut gs.world),
            _ => return RunState::Paused,
        },
    }
    RunState::Running
}
