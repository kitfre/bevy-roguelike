use bevy::prelude::Commands;

pub(crate) fn spawn_player(mut commands: Commands) {
    use crate::components::being::{Being, Health, Name, Player};
    use crate::components::terminal::{Position, Symbol};
    commands
        .spawn()
        .insert(Position { x: 25, y: 25 })
        .insert(Being)
        .insert(Health(100))
        .insert(Name("Player".to_string()))
        .insert(Symbol('@'))
        .insert(Player);
}
