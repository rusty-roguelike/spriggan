use rltk::{VirtualKeyCode, Rltk};
use specs::prelude::*;
use std::cmp::{max, min};
use super::{Position, Player, TileType, State, Map};
use crate::components::Monster;

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut monsters = ecs.write_storage::<Monster>();
    let map = ecs.fetch::<Map>();
    let mut player_position: &Position;

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        player_position = &pos;
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(79 , max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }


    }

    for (mut monster, monster_pos) in (&mut monsters, positions).join() {
        if monster_pos == player_position {
            monster.hp -=1;
        }
    }

}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),

            _ => {}
        },
    }
}
