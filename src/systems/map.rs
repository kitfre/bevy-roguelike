use crate::components::map::{Map, Rect};

pub(crate) fn init_map() -> Map {
    let mut map = Map::square(50);

    let mut rects: Vec<Rect> = Vec::new();

    // TODO: This is awful
    while rects.len() < 4 {
        let rect = Rect::random(0, 50, 0, 50);

        if !rects.iter().any(|r| r.intersects(&rect)) {
            rects.push(rect);
        }
    }

    map.add_rects(rects.into_iter());
    map
}
