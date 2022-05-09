use crate::systems::being::spawn_player;
use crate::systems::map::init_map;
use bevy::prelude::{App, Plugin};

#[derive(Default)]
pub(crate) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(init_map())
            .add_startup_system(spawn_player);
    }
}
