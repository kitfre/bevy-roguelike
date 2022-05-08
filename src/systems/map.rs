use crate::components::map::{Wall, Walls};
use crate::components::terminal::{Position, Symbol};
use bevy::prelude::{Commands, ResMut};

pub(crate) fn spawn_walls(mut commands: Commands, mut walls: ResMut<Walls>) {
    // TODO: make dimensions config
    let mut w = Vec::new();
    for i in 0..50 {
        w.push(
            commands
                .spawn()
                .insert(Position { x: i, y: 0 })
                .insert(Wall)
                .insert(Symbol('#'))
                .id(),
        );

        w.push(
            commands
                .spawn()
                .insert(Position { x: i, y: 49 })
                .insert(Wall)
                .insert(Symbol('#'))
                .id(),
        );

        w.push(
            commands
                .spawn()
                .insert(Position { x: 0, y: i })
                .insert(Wall)
                .insert(Symbol('#'))
                .id(),
        );

        w.push(
            commands
                .spawn()
                .insert(Position { x: 49, y: i })
                .insert(Wall)
                .insert(Symbol('#'))
                .id(),
        );
    }

    *walls = Walls(w);
}
