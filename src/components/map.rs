use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub(crate) struct Wall;

#[derive(Component)]
pub(crate) struct Floor;

#[derive(Default)]
pub(crate) struct Walls(pub Vec<Entity>);
