use crate::components::map::{Map, Rect};
use crate::components::terminal::Position;
use crate::systems::being::spawn_player;
use bevy::prelude::{App, Plugin};

#[derive(Default)]
pub(crate) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let mut map = Map::square(50);

        let rects = vec![Rect {
            start: Position { x: 10, y: 5 },
            width: 4,
            height: 5,
        }];
        map.add_rects(rects.into_iter());

        app.insert_resource(map).add_startup_system(spawn_player);
    }
}
