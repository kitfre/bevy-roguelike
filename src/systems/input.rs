use crate::components::being::Player;
use crate::components::input::Action;

use crate::Position;
use bevy::prelude::{Query, Res, With};

pub(crate) fn handle_input(
    action: Res<Option<Action>>,
    mut positions: Query<&mut Position, With<Player>>,
) {
    use crate::components::input::{Action::Move, Direction::*};

    if let Some(mut pos) = positions.iter_mut().next() {
        match action.as_ref() {
            Some(Move(Left)) => {
                pos.x -= 1;
            }
            Some(Move(Right)) => {
                pos.x += 1;
            }
            Some(Move(Up)) => {
                pos.y += 1;
            }
            Some(Move(Down)) => {
                pos.y -= 1;
            }
            _ => {}
        }
    }
}
