use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::{EventReader, KeyCode, Res, ResMut, SystemLabel};

pub(crate) struct KeyBindings {
    bindings: std::collections::HashMap<KeyCode, Action>,
}

impl KeyBindings {
    pub(crate) fn resolve_keycode(&self, keycode: KeyCode) -> Option<Action> {
        self.bindings.get(&keycode).copied()
    }
}

impl std::default::Default for KeyBindings {
    fn default() -> Self {
        use Action::Move;
        use Direction::*;

        let mut bindings = std::collections::HashMap::new();
        bindings.insert(KeyCode::Left, Move(Left));
        bindings.insert(KeyCode::Right, Move(Right));
        bindings.insert(KeyCode::Up, Move(Up));
        bindings.insert(KeyCode::Down, Move(Down));

        Self { bindings }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) enum Action {
    Move(Direction),
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, SystemLabel)]
pub(crate) struct ActionSystem;

pub(crate) fn action_input_system(
    bindings: Res<KeyBindings>,
    mut action: ResMut<Option<Action>>,
    mut key_reader: EventReader<KeyboardInput>,
) {
    for event in key_reader.iter() {
        if let KeyboardInput {
            key_code: Some(key_code),
            state,
            ..
        } = event
        {
            match bindings.resolve_keycode(*key_code) {
                Some(act) if state.is_pressed() => {
                    *action = Some(act);
                }
                _ => *action = None,
            }
        }
    }
}
