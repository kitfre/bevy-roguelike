use bevy::prelude::{Color, Component};
use bevy_ascii_terminal::{CharFormat, Terminal};

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub(crate) struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Component)]
pub(crate) struct Map {
    grid: Grid,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
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
    pub(crate) fn set(&mut self, pos: Position, tile: Tile) {
        if self.in_bounds(pos) {
            self.grid.0[pos.x as usize][pos.y as usize] = tile;
        }
    }

    pub(crate) fn tile_at(&self, pos: Position) -> Option<Tile> {
        self.grid
            .0
            .get(pos.x as usize)
            .and_then(|tiles| tiles.get(pos.y as usize))
            .copied()
    }

    pub(crate) fn neighbors(&self, pos: Position) -> impl IntoIterator<Item = Position> {
        let mut vec = Vec::new();

        let Position { x, y } = pos;

        if self.in_bounds(Position { x: x + 1, y }) {
            vec.push(Position { x: x + 1, y });
        }

        if x > 0 && self.in_bounds(Position { x: x - 1, y }) {
            vec.push(Position { x: x - 1, y });
        }

        if self.in_bounds(Position { x, y: y + 1 }) {
            vec.push(Position { x, y: y + 1 })
        }

        if y > 0 && self.in_bounds(Position { x, y: y - 1 }) {
            vec.push(Position { x, y: y - 1 });
        }

        vec
    }

    pub(crate) fn rect(width: u32, height: u32) -> Self {
        Self {
            grid: Grid::rect(width, height),
        }
    }

    pub(crate) fn add_rects(&mut self, rects: impl Iterator<Item = Rect>) {
        for rect in rects {
            self.grid.add_rect(&rect);
        }
    }

    pub(crate) fn open(&self, pos: Position) -> bool {
        let x = pos.x as usize;
        let y = pos.y as usize;
        x < self.grid.0.len() && y <= self.grid.0[0].len() && !self.wall_at(pos)
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
                            x: x as u32,
                            y: y as u32,
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

    fn in_bounds(&self, pos: Position) -> bool {
        (pos.x as usize) < self.grid.0.len() && (pos.y as usize) <= self.grid.0[0].len()
    }
}

impl Grid {
    fn rect(width: u32, height: u32) -> Self {
        let tiles: Vec<Vec<Tile>> = (0..width)
            .map(|_| (0..height).map(|_| Tile::Wall).collect())
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Rect {
    // bottom-left corner
    pub start: Position,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub(crate) fn random(x: u32, y: u32, room_width: u32, room_height: u32) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let start = Position {
            x: rng.gen_range(0..x),
            y: rng.gen_range(0..y),
        };

        let mut width = rng.gen_range(1..room_width);
        let mut height = rng.gen_range(1..room_height);

        if start.x + width >= x {
            width = x - start.x;
        }

        if start.y + height >= y {
            height = y - start.y;
        }

        let rect = Self {
            start,
            width,
            height,
        };

        rect
    }

    pub(crate) fn random_interior_position(&self) -> Position {
        use rand::Rng;

        let mut rng = rand::thread_rng();

        let (x_start, x_end) = (self.start.x as u32, self.start.x as u32 + self.width);
        let (y_start, y_end) = (self.start.y as u32, self.start.y as u32 + self.height);

        Position {
            x: rng.gen_range(x_start..x_end),
            y: rng.gen_range(y_start..y_end),
        }
    }
}
