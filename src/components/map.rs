use crate::components::terminal::Position;
use bevy::prelude::{Color, Component};
use bevy_ascii_terminal::{CharFormat, Terminal};

#[derive(Component)]
pub(crate) struct Map {
    grid: Grid,
}

#[derive(Component)]
pub(crate) enum Tile {
    Wall,
    Floor,
}

impl Tile {
    fn ascii(&self) -> char {
        match self {
            Self::Wall => '#',
            Self::Floor => '.',
        }
    }
}

#[derive(Component)]
pub(crate) struct Grid(Vec<Vec<Tile>>);

impl Map {
    pub(crate) fn square(size: usize) -> Self {
        let mut grid = Grid::square(size);

        // set the borders to be walls
        for i in 0..size {
            grid.0[0][i] = Tile::Wall;
            grid.0[i][0] = Tile::Wall;
            grid.0[size - 1][i] = Tile::Wall;
            grid.0[i][size - 1] = Tile::Wall;
        }

        Self { grid }
    }

    pub(crate) fn open(&self, pos: Position) -> bool {
        let x = pos.x as usize;
        let y = pos.y as usize;
        x < self.grid.0.len()
            && pos.x >= 0
            && y <= self.grid.0[0].len()
            && pos.y >= 0
            && !self.wall_at(pos)
    }

    pub(crate) fn wall_at(&self, pos: Position) -> bool {
        let x = pos.x as usize;
        let y = pos.y as usize;

        match self.grid.0[x][y] {
            Tile::Wall => true,
            _ => false,
        }
    }

    pub(crate) fn render(&self, terminal: &mut Terminal) {
        self.tiles().for_each(|(x, y, tile)| {
            if terminal.is_in_bounds([x as i32, y as i32]) {
                terminal.put_char_formatted(
                    [x as i32, y as i32],
                    tile.ascii(),
                    CharFormat::new(Color::WHITE, Color::BLACK),
                );
            }
        })
    }

    fn tiles(&self) -> impl Iterator<Item = (usize, usize, &Tile)> {
        self.grid
            .0
            .iter()
            .enumerate()
            .map(|(x, row)| row.iter().enumerate().map(move |(y, tile)| (x, y, tile)))
            .flatten()
    }
}

impl Grid {
    fn square(size: usize) -> Self {
        let tiles: Vec<Vec<Tile>> = (0..size)
            .map(|_| (0..size).map(|_| Tile::Floor).collect())
            .collect();

        Self(tiles)
    }
}
