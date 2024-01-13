use image::{Rgb, RgbImage};
use robotics_lib::world::tile::{Content, Tile, TileType};
use crate::world_gen::PreWorld;
impl PreWorld {
    pub fn export_to_image(&self, filename: &str)->Result<(),()> {
        let (width, height) = (self.size, self.size);


        let mut image = RgbImage::new(width as u32, height as u32);

        for x in 0..width {
            for y in 0..height {
                let tile_color = color_for_tile(&self.tiles[x][y]);
                image.put_pixel(x as u32, y as u32, tile_color);
            }
        }

        // Save the image to a file
        image.save(filename)
    }
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

pub fn color_for_tiletype(tile: &TileType) -> Rgb<u8> {
    // TODO Adjust colors
    match tile {
        TileType::DeepWater => Rgb([0,0,125]),
        TileType::Grass => Rgb([124,252,0]),
        TileType::Sand => Rgb([246,215,176]),
        TileType::ShallowWater => Rgb([35,137,218]),
        TileType::Mountain => Rgb([90, 75, 65]),
        TileType::Lava => Rgb([207, 16, 32]),

        TileType::Street => Rgb([50,50,50]),
        TileType::Hill => Rgb([0,153,51]),
        _ => Rgb([0,0,0]),
    }
}
pub fn color_for_tile(tile:&Tile)->Rgb<u8>{
    let tiletype_color = color_for_tiletype(&tile.tile_type);
    let content_color = color_for_content(&tile.content);
    match content_color{
        None => {tiletype_color}
        Some(color) => {color}
    }
}