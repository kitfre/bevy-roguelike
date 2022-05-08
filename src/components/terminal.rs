use bevy::prelude::Component;

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub(crate) struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, PartialEq, Clone, Copy)]
pub(crate) struct Symbol(pub char);
