use super::Generator;
use crate::components::map::{Map, Position, Rect, Tile};

pub(crate) struct RoomsAndHallways {
    pub width: u32,
    pub height: u32,
    pub room_width: u32,
    pub room_height: u32,
    pub max_rooms: u32,
}

impl Generator for RoomsAndHallways {
    type Output = Map;

    // TODO: This is awful
    fn generate(&mut self) -> Map {
        use itertools::Itertools;

        let mut map = Map::rect(self.width, self.height);
        let mut rects: Vec<Rect> = Vec::new();

        // Generate rooms
        while rects.len() < self.max_rooms as usize {
            rects.push(Rect::random(
                self.width,
                self.height,
                self.room_width,
                self.room_height,
            ));
        }

        // Pick anchors for each room
        let anchors = rects
            .iter()
            .map(Rect::random_interior_position)
            .collect::<Vec<_>>();

        let points = anchors.iter().cartesian_product(anchors.iter());

        // Build paths between all anchor points
        // TODO: Better distance and heuristics
        let h = |pos, map: &Map| {
            // walls are more costly to tunnel through
            match map.tile_at(pos) {
                Some(Tile::Wall) => 3,
                Some(Tile::Floor) => 1,
                None => std::u32::MAX,
            }
        };

        // Taxi cab distance
        let d = |a: Position, b: Position| a.x.abs_diff(b.x) + a.y.abs_diff(b.y);

        let paths = points
            .filter_map(|(&start, &end)| shortest_path(start, end, &map, h, d))
            .collect::<Vec<_>>();

        // add the rooms
        map.add_rects(rects.into_iter());

        // and the paths
        for path in paths {
            for pos in path {
                map.set(pos, Tile::Floor);
            }
        }

        map
    }
}

fn shortest_path(
    start: Position,
    goal: Position,
    map: &Map,
    h: impl Fn(Position, &Map) -> u32,
    d: impl Fn(Position, Position) -> u32,
) -> Option<Vec<Position>> {
    let mut open = std::collections::BinaryHeap::new();
    open.push(Node {
        cost: h(start, map),
        pos: start,
    });

    let mut came_from: std::collections::HashMap<Position, Position> =
        std::collections::HashMap::new();

    let mut scores = std::collections::HashMap::new();
    scores.insert(start, 0);
    while !open.is_empty() {
        if let Some(curr) = open.pop() {
            if curr.pos == goal {
                // TODO: reconstruct path
                return reconstruct_path(came_from, curr.pos);
            }

            for neighbor in map.neighbors(curr.pos) {
                let potential_score =
                    scores.get(&curr.pos).copied().unwrap_or(std::u32::MAX) + d(curr.pos, neighbor);
                if potential_score < scores.get(&neighbor).copied().unwrap_or(std::u32::MAX) {
                    came_from.insert(neighbor, curr.pos);
                    scores.insert(neighbor, potential_score);
                    open.push(Node {
                        cost: h(neighbor, map),
                        pos: neighbor,
                    })
                }
            }
        }
    }
    None
}

fn reconstruct_path(
    paths: std::collections::HashMap<Position, Position>,
    start: Position,
) -> Option<Vec<Position>> {
    let mut path = vec![start];

    let mut curr = start;
    while paths.contains_key(&curr) {
        match paths.get(&curr) {
            None => {
                return None;
            }
            Some(p) => {
                curr = *p;
                path.push(*p);
            }
        }
    }

    Some(path)
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Node {
    cost: u32,
    pos: Position,
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
