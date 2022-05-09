use crate::components::being::Player;
use crate::components::input::{Action, KeyBindings};
use crate::components::map::{Map, Position};

use bevy::input::keyboard::KeyboardInput;
use bevy::input::ElementState;
use bevy::prelude::{EventReader, Input, Query, Res, ResMut, With};

pub(crate) fn handle_input(
    action: Res<Input<Action>>,
    map: Res<Map>,
    mut positions: Query<&mut Position, With<Player>>,
) {
    use crate::components::input::{Action::Move, Direction::*};
    if !action.is_changed() {
        return;
    }

    if let Some(mut pos) = positions.iter_mut().next() {
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

        // check if the new position collides with a wall
        if map.in_bounds(new_pos) && map.open(new_pos) {
            *pos = new_pos;
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
