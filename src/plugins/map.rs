use crate::systems::being::spawn_player;
use crate::systems::map::spawn_map;
use bevy::prelude::{App, Plugin};

#[derive(Default)]
pub(crate) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_map)
            .add_startup_system(spawn_player);
    }
}
