use crate::components::being::Player;
use crate::components::input::{Action, KeyBindings};
use crate::components::map::Walls;
use crate::components::terminal::Position;

use bevy::input::keyboard::KeyboardInput;
use bevy::input::ElementState;
use bevy::prelude::{Entity, EventReader, Input, Query, Res, ResMut, With};
use bevy_ascii_terminal::Terminal;

pub(crate) fn handle_input(
    action: Res<Input<Action>>,
    walls: Res<Walls>,
    mut players: Query<Entity, With<Player>>,
    mut positions: Query<&mut Position>,
    term_q: Query<&Terminal>,
) {
    use crate::components::input::{Action::Move, Direction::*};

    let terminal = term_q.get_single().unwrap();

    let wall_positions = walls
        .0
        .iter()
        .map(|&p| positions.get(p).unwrap().clone())
        .collect::<Vec<Position>>();

    if let Some(player) = players.iter_mut().next() {
        let mut pos = positions.get_mut(player).unwrap();
        let new_pos = if action.pressed(Move(Left)) {
            Position {
                x: pos.x - 1,
                y: pos.y,
            }
        } else if action.pressed(Move(Right)) {
            Position {
                x: pos.x + 1,
                y: pos.y,
            }
        } else if action.pressed(Move(Up)) {
            Position {
                x: pos.x,
                y: pos.y + 1,
            }
        } else if action.pressed(Move(Down)) {
            Position {
                x: pos.x,
                y: pos.y - 1,
            }
        } else {
            pos.clone()
        };

        if terminal.is_in_bounds([new_pos.x, new_pos.y]) {
            // check if the new position collides with a wall

            if wall_positions.into_iter().find(|&p| p == new_pos).is_none() {
                *pos = new_pos;
            }
        }
    }
}

pub(crate) fn action_input_system(
    bindings: Res<KeyBindings>,
    mut action: ResMut<Input<Action>>,
    mut key_reader: EventReader<KeyboardInput>,
) {
    for event in key_reader.iter() {
        if let KeyboardInput {
            key_code: Some(key_code),
            state,
            ..
        } = event
        {
            match (bindings.resolve_keycode(*key_code), state) {
                (Some(act), ElementState::Pressed) => action.press(act),
                (Some(act), ElementState::Released) => action.release(act),
                _ => {}
            }
        }
    }
}
