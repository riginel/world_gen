use image::{Rgb, RgbImage};
use robotics_lib::world::tile::Content;
use crate::tile::{_TileType, color_for_tile, PreWorld};

pub fn export_to_image(map: &PreWorld, filename: &str) {
    let (width,height) = map.size.as_tuple();


    let mut image = RgbImage::new(width as u32, height as u32);

    for x in 0..width {
        for y in 0..height {
            let tile_color = color_for_tile(map.tiles[x][y]);
            let content_color = color_for_content(&map.contents[x][y]);
            if let Some(content_color) = content_color{
                image.put_pixel(x as u32, y as u32, content_color);
            }else {
                image.put_pixel(x as u32, y as u32, tile_color);
            }


        }
    }

    // Save the image to a file
    image.save(filename).expect("Failed to save image");
}
fn color_for_content(content:&Content)-> Option<Rgb<u8>>{
    match content{
        Content::Rock(_) => {Some(Rgb([58, 58, 59]))}
        Content::Tree(_) => {Some(Rgb([26, 41, 24]))}
        Content::Garbage(_) => {Some(Rgb([25, 26, 24]))}
        Content::Fire => {Some(Rgb([107, 37, 48]))}
        Content::Coin(_) => {Some(Rgb([107, 99, 35]))}
        Content::Bin(_) => {Some(Rgb([51, 50, 45]))}
        Content::Crate(_) => {Some(Rgb([56, 27, 14]))}
        Content::Bank(_) => {Some(Rgb([72, 14, 74]))}
        Content::Water(_) => {None}
        Content::Market(_) => {Some(Rgb([85, 99, 36]))}
        Content::Fish(_) => {Some(Rgb([59, 62, 69]))}
        Content::Building => {Some(Rgb([49, 50, 51]))}
        Content::Bush(_) => {Some(Rgb([89, 217, 91]))}
        Content::JollyBlock(_) => {None}
        Content::Scarecrow => {None}
        Content::None =>None
    }
}