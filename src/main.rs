use image::{GenericImage, GenericImageView};
use rand::prelude::*;

mod helpers;
use helpers::position_helpers::{Position, get_closest_point};
use helpers::error_helpers::{get_zone_average, get_zone_mse};


struct Voronoi {
    /// The list of each of the points
    points: Vec<Position>,
    /// Each list in zones contains all the pixels present in the
    /// zone of the point at the same index
    zones: Vec<Vec<Position>>,
}

fn main() {
    let src = image::open("images/girl_with_pearl.jpg").unwrap();
    let mut voronoi = initiate_voronoi(&src, 10);
    for i in 0..2000 {
        println!("Entering round {:}", i);
        add_point_to_voronoi(&mut voronoi, &src);
        recalculate_voronoi_zones(&mut voronoi, &src);
        if i%50 == 0 {
            save_voronoi(&voronoi, &src, format!("results/voronoi_{:}.png", i));
        }
    }
    let img = create_voronoi(&src);
    img.save("results/voronoi.png");
}

fn initiate_voronoi(src: &image::DynamicImage, num_points: u32) -> Voronoi {
    let mut voronoi = Voronoi {
        points: Vec::new(),
        zones: Vec::new(),
    };
    let (image_width, image_height) = src.dimensions();
    let mut rng = rand::thread_rng();
    for _ in 0..num_points {
        let x = rng.gen_range(0, image_width as u32);
        let y = rng.gen_range(0, image_height as u32);
        voronoi.points.push( Position {x: x, y: y} );
        voronoi.zones.push(Vec::new());
    }
    recalculate_voronoi_zones(&mut voronoi, &src);
    voronoi
}

fn add_point_to_voronoi(voronoi: &mut Voronoi, src: &image::DynamicImage) {
    /// This function finds the zone with the highest mse when compared
    /// to the source image, and replaces that zones point with two points
    /// in the same zone
    let mut highest_mse = 0.0;
    let mut highest_point_index = 0 as usize;
    for (i, zone) in voronoi.zones.iter().enumerate() {
        let zone_mse = get_zone_mse(&zone, &src);
        if zone_mse > highest_mse {
            highest_mse = zone_mse;
            highest_point_index = i;
        }
    }
    let mut rng = rand::thread_rng();
    let high_zone = &voronoi.zones[highest_point_index];
    let rand_point_index_1 = rng.gen_range(0, high_zone.len() as u32);
    let rand_point_index_2 = rng.gen_range(0, high_zone.len() as u32);
    let rand_point_1 = high_zone[rand_point_index_1 as usize].clone();
    let rand_point_2 = high_zone[rand_point_index_2 as usize].clone();
    // voronoi.points[highest_point_index] = rand_point_1;
    voronoi.points.push(rand_point_2);
    voronoi.zones.push(Vec::new());
    println!("Highest mse in zone : {:}", highest_point_index);
}

fn recalculate_voronoi_zones(voronoi: &mut Voronoi, src: &image::DynamicImage) {
    let (image_width, image_height) = src.dimensions();
    for i in 0..voronoi.points.len() {
        voronoi.zones[i].clear();
    }
    for x in 0..image_width {
        for y in 0..image_height {
            let pos = Position { x:x, y:y };
            let closest_index = get_closest_point(&pos, &voronoi.points);
            voronoi.zones[closest_index].push(pos);
        }
    }
}

fn save_voronoi(voronoi: &Voronoi, src: &image::DynamicImage, filename: String) {
    let (image_width, image_height) = src.dimensions();
    let mut dest = image::DynamicImage::new_rgb8(image_width, image_height);
    for zone in &voronoi.zones {
        let colour = get_zone_average(&zone, &src);
        for point in zone {
            dest.put_pixel(point.x, point.y, image::Rgba{ 0:[colour[0], colour[1], colour[2], 255 as u8]});
        }
    }
    dest.save(filename);
}

fn create_voronoi(src: &image::DynamicImage) -> image::DynamicImage {
    let (image_width, image_height) = src.dimensions();
    let mut dest = image::DynamicImage::new_rgb8(image_width, image_height);
    let mut rng = rand::thread_rng();
    let mut points = Vec::new();
    let mut colours = Vec::new();
    for _ in 0..10 {
        let x = rng.gen_range(0, image_width as u32);
        let y = rng.gen_range(0, image_height as u32);
        points.push( Position {x: x, y: y} );
        let r = rng.gen_range(0, 255 as u8);
        let g = rng.gen_range(0, 255 as u8);
        let b = rng.gen_range(0, 255 as u8);
        colours.push( [r, g, b] );
    }
    for x in 0..image_width {
        for y in 0..image_height {
            let closest_index = get_closest_point(&Position {x:x, y:y}, &points);
            let colour = colours[closest_index];
            dest.put_pixel(x, y, image::Rgba{ 0: [colour[0] as u8, colour[1] as u8, colour[2] as u8, 255 as u8]});
        }
    }
    dest
}

