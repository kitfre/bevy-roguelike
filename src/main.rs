mod components;
mod plugins;
mod systems;

use crate::plugins::{ActionPlugin, GridPlugin};
use bevy::prelude::*;
use bevy_ascii_terminal::TerminalPlugin;
use bevy_tiled_camera::TiledCameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_plugin(TiledCameraPlugin)
        .add_plugin(ActionPlugin)
        .add_plugin(GridPlugin)
        .add_startup_system(crate::systems::being::spawn_player)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
