use image::{GenericImage, GenericImageView};
use rand::prelude::*;

mod helpers;
use helpers::helpers::{Position, get_closest_point};


fn main() {
    let src = image::open("images/girl_with_pearl.jpg").unwrap();
    let img = voronoi(&src);
    img.save("results/voronoi.png");
}

fn voronoi(src: &image::DynamicImage) -> image::DynamicImage {
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
            let closest_index = get_closest_point(Position {x:x, y:y}, &points);
            let colour = colours[closest_index];
            dest.put_pixel(x, y, image::Rgba{ 0: [colour[0] as u8, colour[1] as u8, colour[2] as u8, 255 as u8]});
        }
    }
    dest
}

