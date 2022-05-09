use crate::components::map::Map;
use crate::components::terminal::{Position, Symbol};
use bevy::prelude::{Color, Query};
use bevy_ascii_terminal::CharFormat;
use bevy_ascii_terminal::Terminal;

pub(crate) fn update_position(
    map_q: Query<&Map>,
    q: Query<(&Position, &Symbol)>,
    mut term_q: Query<&mut Terminal>,
) {
    let mut terminal = term_q.get_single_mut().unwrap();
    terminal.clear();

    // draw the map first
    let map = map_q.get_single().unwrap();
    map.render(terminal.as_mut());

    // draw entities over it
    for (pos, symbol) in q.iter() {
        if terminal.is_in_bounds([pos.x, pos.y]) {
            terminal.put_char_formatted(
                [pos.x, pos.y],
                symbol.0,
                CharFormat::new(Color::WHITE, Color::BLACK),
            );
        }
    }
}
