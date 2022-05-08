use bevy::prelude::Component;

#[derive(Component)]
pub(crate) struct Being;

#[derive(Component)]
pub(crate) struct Health(pub u32);

#[derive(Component)]
pub(crate) struct Name(pub String);

#[derive(Component)]
pub(crate) struct Player;
