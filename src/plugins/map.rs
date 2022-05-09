use crate::procgen::{map::RoomsAndHallways, Generator};
use crate::systems::being::spawn_player;
use bevy::prelude::{App, Plugin};

#[derive(Default)]
pub(crate) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let mut gen = RoomsAndHallways {
            width: 50,
            height: 50,
            room_width: 20,
            room_height: 20,
            max_rooms: 10,
        };

        app.insert_resource(gen.generate())
            .add_startup_system(spawn_player);
    }
}
