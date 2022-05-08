mod input;
mod plugins;

use crate::input::Action;
use crate::plugins::ActionPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugin(ActionPlugin)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)
        .add_system(size_scaling)
        .add_system(update_position)
        .add_system(handle_input)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

#[derive(Component)]
struct Character;

#[derive(Component)]
struct Health(u32);

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Player;

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

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "c",
                TextStyle {
                    font: asset_server.load("fonts/arcade-classic.ttf"),
                    font_size: 36.0,
                    color: Color::BLACK,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(Position { x: 100, y: 100 })
        .insert(Size {
            height: 1.,
            width: 1.,
        })
        .insert(Health(100))
        .insert(Name("Player".to_string()))
        .insert(Player);
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

fn handle_input(action: Res<Option<Action>>, mut positions: Query<&mut Position, With<Player>>) {
    use crate::input::{Action::Move, Direction::*};

    if let Some(mut pos) = positions.iter_mut().next() {
        match action.as_ref() {
            Some(Move(Left)) => {
                pos.x -= 1;
            }
            Some(Move(Right)) => {
                pos.x += 1;
            }
            Some(Move(Up)) => {
                pos.y += 1;
            }
            Some(Move(Down)) => {
                pos.y -= 1;
            }
            _ => {}
        }
    }
}
