use bevy::prelude::{
    default, AssetServer, Color, Commands, HorizontalAlign, Res, Text, Text2dBundle, TextAlignment,
    TextStyle,
};

pub(crate) fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    use crate::components::being::{Being, Health, Name, Player};
    use crate::{Position, Size};
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
        .insert(Being)
        .insert(Health(100))
        .insert(Name("Player".to_string()))
        .insert(Player);
}
