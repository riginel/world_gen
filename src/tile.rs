use image::Rgb;
use crate::pathfinding::a_star::Walkable;

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    Grass,
    Water,
    DeepWater,
    Sand,
    Mountain,
    Lava,
    Zone(usize), // Used in intermediate world generation.
    Road,
    None,
    Hill

}
pub fn trans(z: f64) -> Tile {
    // Trans function (because we are using RustLang).
    // Perlin Noise return a -1.0 <= f64 <= 1.0,
    // This function normalize(?) the range between [0.0, 1.0]
    // and map each value to a TileType using the below rules.
    // TODO adjust the conversion rules.
    match z / 2.0 + 0.5 {
        w if w < 0.32 => {Tile::DeepWater}
        w if w < 0.42 => {Tile::Water}
        w if w < 0.52 => {Tile::Zone(0)}
        w if w < 0.7 => {Tile::Grass}
        w if w <= 0.81 => {Tile::Zone(1)}
        w if w <= 1.0 => {Tile::Zone(2)}
        _ => {Tile::Lava}
    }
}
//for pathfinding
impl Walkable for Tile{
    fn cost(&self) -> u32 {
        match self{
            Tile::Grass => {1}
            Tile::Water => {3}
            Tile::DeepWater => {10}
            Tile::Sand => {1}
            Tile::Mountain => {500}
            Tile::Lava => {100000}
            Tile::Road => {0}
            _ => {1}
        }
    }
}
pub fn color_for_tile(tile: Tile) -> Rgb<u8> {
    // TODO Adjust colors
    match tile {
        Tile::DeepWater => Rgb([0,0,125]),
        Tile::Grass => Rgb([124,252,0]),
        Tile::Sand => Rgb([246,215,176]),
        Tile::Water => Rgb([35,137,218]),
        Tile::Mountain => Rgb([90, 75, 65]),
        Tile::Lava => Rgb([207, 16, 32]),
        //Tile::Tree => Rgb([66, 105, 47]),
        Tile::Road => Rgb([50,50,50]),
        Tile::Zone(0) => Rgb([0, 0, 0]),
        Tile::Zone(1) => Rgb([0, 125, 0]),
        Tile::Zone(2) => Rgb([125, 0, 0]),
        Tile::None => Rgb([255, 255, 255]),
        Tile::Hill => Rgb([0,153,51]),
        _ => Rgb([0,0,0]),
    }
}
