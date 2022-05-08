use crate::components::map::Walls;
use crate::systems::being::spawn_player;
use crate::systems::map::spawn_walls;
use bevy::prelude::{App, Plugin};

#[derive(Default)]
pub(crate) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Walls>()
            .add_startup_system(spawn_walls)
            .add_startup_system(spawn_player);
    }
}
