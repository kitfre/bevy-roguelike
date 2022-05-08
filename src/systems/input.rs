use crate::components::being::Player;
use crate::components::input::{Action, KeyBindings};
use crate::components::terminal::Position;

use bevy::input::keyboard::KeyboardInput;
use bevy::input::ElementState;
use bevy::prelude::{EventReader, Input, Query, Res, ResMut, With};

pub(crate) fn handle_input(
    action: Res<Input<Action>>,
    mut positions: Query<&mut Position, With<Player>>,
) {
    use crate::components::input::{Action::Move, Direction::*};

    if let Some(mut pos) = positions.iter_mut().next() {
        if action.pressed(Move(Left)) {
            pos.x -= 1;
        }

        if action.pressed(Move(Right)) {
            pos.x += 1;
        }
        if action.pressed(Move(Up)) {
            pos.y += 1;
        }
        if action.pressed(Move(Down)) {
            pos.y -= 1;
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
