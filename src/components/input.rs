use bevy::prelude::{KeyCode, SystemLabel};

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
