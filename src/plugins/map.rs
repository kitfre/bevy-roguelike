use crate::procgen::{map::RoomsAndHallways, Generator};
use crate::systems::being::spawn_player;
use bevy::prelude::{App, Plugin};

#[derive(Default)]
pub(crate) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RoomsAndHallways::generate())
            .add_startup_system(spawn_player);
    }
}
