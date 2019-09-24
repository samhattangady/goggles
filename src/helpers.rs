pub mod helpers {
    pub struct Position {
        pub x: u32,
        pub y: u32,
    }

    pub fn get_closest_point(position: Position, points: &Vec<Position>) -> usize {
        let mut closest_index = 0;
        let mut closest_distance = std::f32::MAX;
        for (i, point) in points.iter().enumerate() {
            let distance = get_distance(&position, point);
            if distance < closest_distance {
                closest_distance = distance;
                closest_index = i;
            }
        }
        closest_index
    }

    fn get_distance(p1: &Position, p2: &Position) -> f32 {
        let dx = std::cmp::max(p1.x, p2.x) - std::cmp::min(p1.x, p2.x);
        let dy = std::cmp::max(p1.y, p2.y) - std::cmp::min(p1.y, p2.y);
        ((dx.pow(2) + dy.pow(2)) as f32).powf(0.5)
    }
}

