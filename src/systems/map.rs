use crate::components::map::Map;
use bevy::prelude::Commands;

pub(crate) fn spawn_map(mut commands: Commands) {
    commands.spawn().insert(Map::square(50));
}
