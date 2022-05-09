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
        let grid = Grid::square(size);

        Self { grid }
    }

    pub(crate) fn add_rects(&mut self, rects: impl Iterator<Item = Rect>) {
        for rect in rects {
            self.grid.add_rect(&rect);
        }
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

    pub(crate) fn pick_open_space(&self) -> Option<Position> {
        use rand::seq::IteratorRandom;

        let mut rng = rand::thread_rng();
        self.grid
            .0
            .iter()
            .enumerate()
            .map(|(x, tiles)| {
                tiles
                    .iter()
                    .enumerate()
                    .filter_map(move |(y, tile)| match tile {
                        Tile::Wall => None,
                        Tile::Floor => Some(Position {
                            x: x as i32,
                            y: y as i32,
                        }),
                    })
            })
            .flatten()
            .choose(&mut rng)
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
            .map(|_| (0..size).map(|_| Tile::Wall).collect())
            .collect();

        Self(tiles)
    }

    fn add_rect(&mut self, rect: &Rect) {
        let x_start = rect.start.x as usize;
        let y_start = rect.start.y as usize;

        let x_end = x_start + rect.width as usize;
        let y_end = y_start + rect.height as usize;

        for x in x_start..x_end {
            for y in y_start..y_end {
                self.0[x][y] = Tile::Floor;
            }
        }
    }
}

pub(crate) struct Rect {
    // bottom-left corner
    pub start: Position,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub(crate) fn random(min_x: u32, max_x: u32, min_y: u32, max_y: u32) -> Self {
        use rand::Rng;

        let mut rng = rand::thread_rng();

        let start = Position {
            x: rng.gen_range(min_x..max_x) as i32,
            y: rng.gen_range(min_y..max_y) as i32,
        };

        let width = rng.gen_range(min_x..max_x - start.x as u32);
        let height = rng.gen_range(min_y..max_y - start.y as u32);

        Self {
            start,
            width,
            height,
        }
    }

    pub(crate) fn intersects(&self, other: &Rect) -> bool {
        let (x_start, x_end) = (self.start.x as u32, self.start.x as u32 + self.width);
        let (y_start, y_end) = (self.start.y as u32, self.start.y as u32 + self.height);

        let (other_x_start, other_x_end) =
            (other.start.x as u32, other.start.x as u32 + other.width);
        let (other_y_start, other_y_end) =
            (other.start.y as u32, other.start.y as u32 + other.height);

        !(x_end < other_x_start
            || other_x_end < x_start
            || y_end < other_y_start
            || other_y_end < y_start)
    }
}
