pub mod position_helpers {
    #[derive(Debug, Clone, Copy)]
    pub struct Position {
        pub x: u32,
        pub y: u32,
    }

    pub fn get_closest_point(position: &Position, points: &Vec<Position>) -> usize {
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

pub mod error_helpers {
    use image::{GenericImage, GenericImageView};
    use super::position_helpers::Position;

    pub fn get_zone_mse(zone: &Vec<Position>, src: &image::DynamicImage) -> f32 {
        if zone.len() == 0 {
            return 0.0;
        }
        let zone_average = get_zone_average(&zone, &src);
        let mut zone_mse = 0.0;
        for point in zone {
            let pixel = src.get_pixel(point.x, point.y);
            let pixel_data = [pixel.0[0], pixel.0[1], pixel.0[2]];
            zone_mse += get_pixel_mse(&pixel_data, &zone_average)
        }
        zone_mse /= zone.len() as f32;
        zone_mse
    }

    pub fn get_zone_average(zone: &Vec<Position>, src: &image::DynamicImage) -> [u8; 3] {
        if zone.len() == 0 {
            return [0 as u8, 0 as u8, 0 as u8];
        }
        let mut zone_sum = [0 as u32, 0 as u32, 0 as u32];
        for point in zone {
            let pixel = src.get_pixel(point.x, point.y);
            zone_sum[0] += pixel.0[0] as u32;
            zone_sum[1] += pixel.0[1] as u32;
            zone_sum[2] += pixel.0[2] as u32;
        }
        zone_sum = [zone_sum[0]/zone.len() as u32, zone_sum[1]/zone.len() as u32, zone_sum[2]/zone.len() as u32];
        [zone_sum[0] as u8, zone_sum[1] as u8, zone_sum[2] as u8]
        
    }

    fn get_pixel_mse(p1: &[u8; 3], p2: &[u8; 3]) -> f32 {
         let mut square_error = 0.0;
         square_error += (p1[0] as f32 - p2[0] as f32).powf(2.0);
         square_error += (p1[1] as f32 - p2[1] as f32).powf(2.0);
         square_error += (p1[2] as f32 - p2[2] as f32).powf(2.0);
         square_error
    }
}
