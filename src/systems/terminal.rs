use crate::components::terminal::{Position, Symbol};
use bevy::prelude::{Color, Query};
use bevy_ascii_terminal::CharFormat;
use bevy_ascii_terminal::Terminal;

pub(crate) fn update_position(
    mut q: Query<(&Position, &Symbol)>,
    mut term_q: Query<&mut Terminal>,
) {
    let mut terminal = term_q.get_single_mut().unwrap();
    terminal.clear();
    for (pos, symbol) in q.iter_mut() {
        if terminal.is_in_bounds([pos.x, pos.y]) {
            terminal.put_char_formatted(
                [pos.x, pos.y],
                symbol.0,
                CharFormat::new(Color::WHITE, Color::BLACK),
            );
        }
    }
}
