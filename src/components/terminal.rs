use bevy::prelude::Component;

#[derive(Component, PartialEq, Clone, Copy)]
pub(crate) struct Symbol(pub char);
