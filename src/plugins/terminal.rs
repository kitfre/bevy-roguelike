use crate::systems::terminal::update_position;
use bevy::prelude::{App, Commands, CoreStage, Plugin, SystemSet};

#[derive(Default)]
pub(crate) struct TerminalRenderPlugin;

impl Plugin for TerminalRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new().with_system(update_position),
        );
    }
}

fn setup(mut commands: Commands) {
    use bevy_ascii_terminal::*;
    use bevy_tiled_camera::*;
    let size = [50; 2];

    let term_bundle = TerminalBundle::new().with_size(size);
    commands.spawn_bundle(term_bundle);
    commands.spawn_bundle(TiledCameraBundle::new().with_tile_count(size));
}
