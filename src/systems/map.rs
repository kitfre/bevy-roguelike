use crate::components::map::{Map, Rect};
use crate::components::terminal::Position;
use bevy::prelude::Commands;

pub(crate) fn spawn_map(mut commands: Commands) {
    let mut map = Map::square(50);

    let rects = vec![Rect {
        start: Position { x: 10, y: 5 },
        width: 4,
        height: 5,
    }];
    map.add_rects(rects.into_iter());
    commands.spawn().insert(map);
}
