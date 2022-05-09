use crate::components::map::Map;
use bevy::prelude::{Commands, Res};

pub(crate) fn spawn_player(mut commands: Commands, map: Res<Map>) {
    use crate::components::being::{Being, Health, Name, Player};
    use crate::components::terminal::Symbol;

    // pick a random open space as the spawn point
    if let Some(player_spawn) = map.pick_open_space() {
        commands
            .spawn()
            .insert(player_spawn)
            .insert(Being)
            .insert(Health(100))
            .insert(Name("Player".to_string()))
            .insert(Symbol('@'))
            .insert(Player);
    }
}
