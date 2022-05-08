mod components;
mod plugins;
mod systems;

use crate::plugins::ActionPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugin(ActionPlugin)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(crate::systems::being::spawn_player)
        .add_system(size_scaling)
        .add_system(update_position)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, PartialEq, Clone, Copy)]
struct Size {
    height: f32,
    width: f32,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn update_position(mut q: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(pos.x as f32, pos.y as f32, 0.0);
    }
}

fn size_scaling(mut q: Query<(&Size, &mut Transform), With<Text>>) {
    for (size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(size.height, size.width, 1.0);
    }
}
