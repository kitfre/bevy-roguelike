use super::Generator;
use crate::components::map::{Map, Rect};

pub(crate) struct RoomsAndHallways;

impl Generator for RoomsAndHallways {
    type Output = Map;

    // TODO: This is awful
    fn generate() -> Map {
        let mut map = Map::square(50);

        let mut rects: Vec<Rect> = Vec::new();

        while rects.len() < 4 {
            let rect = Rect::random(0, 50, 0, 50);

            if !rects.iter().any(|r| r.intersects(&rect)) {
                rects.push(rect);
            }
        }

        let start = rects[0];
        for i in 1..4 {
            map.connect(start.start, rects[i].start);
        }

        map.add_rects(rects.into_iter());
        map
    }
}
