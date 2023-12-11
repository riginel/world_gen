use image::RgbImage;
use crate::tile::Tile;

pub fn export_to_image(map: &Vec<Vec<Tile>>, filename: &str) {

    let width = map.len();
    let height = map[0].len();

    let mut image = RgbImage::new(width as u32, height as u32);

    for x in 0..width {
        for y in 0..height {
            let color = color_for_tile(map[x][y]);
            image.put_pixel(x as u32, y as u32, color);
        }
    }

    // Save the image to a file
    image.save(filename).expect("Failed to save image");
}