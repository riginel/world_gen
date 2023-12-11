use std::ops::Range;
use image::Rgb;
use robotics_lib::world::tile::Content;
use crate::pathfinding::a_star::{Point, Walkable};

#[derive(Debug, Copy, Clone,PartialEq,Eq,Hash)]
pub enum _TileType {
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
pub fn trans(z: f64) -> _TileType {
    // Trans function (because we are using RustLang).
    // Perlin Noise return a -1.0 <= f64 <= 1.0,
    // This function normalize(?) the range between [0.0, 1.0]
    // and map each value to a TileType using the below rules.
    // TODO adjust the conversion rules.
    match z / 2.0 + 0.5 {
        w if w < 0.32 => { _TileType::DeepWater}
        w if w < 0.42 => { _TileType::Water}
        w if w < 0.52 => { _TileType::Zone(0)}
        w if w < 0.7 => { _TileType::Grass}
        w if w <= 0.81 => { _TileType::Zone(1)}
        w if w <= 1.0 => { _TileType::Zone(2)}
        _ => { _TileType::Lava}
    }
}
//for pathfinding
impl Walkable for _TileType {
    fn cost(&self) -> u32 {
        match self{
            _TileType::Grass => {1}
            _TileType::Water => {3}
            _TileType::DeepWater => {10}
            _TileType::Sand => {1}
            _TileType::Mountain => {500}
            _TileType::Lava => {100000}
            _TileType::Road => {0}
            _ => {1}
        }
    }
}
pub fn color_for_tile(tile: _TileType) -> Rgb<u8> {
    // TODO Adjust colors
    match tile {
        _TileType::DeepWater => Rgb([0,0,125]),
        _TileType::Grass => Rgb([124,252,0]),
        _TileType::Sand => Rgb([246,215,176]),
        _TileType::Water => Rgb([35,137,218]),
        _TileType::Mountain => Rgb([90, 75, 65]),
        _TileType::Lava => Rgb([207, 16, 32]),
        //Tile::Tree => Rgb([66, 105, 47]),
        _TileType::Road => Rgb([50,50,50]),
        _TileType::Zone(0) => Rgb([0, 0, 0]),
        _TileType::Zone(1) => Rgb([0, 125, 0]),
        _TileType::Zone(2) => Rgb([125, 0, 0]),
        _TileType::None => Rgb([255, 255, 255]),
        _TileType::Hill => Rgb([0,153,51]),
        _ => Rgb([0,0,0]),
    }
}

pub struct PreWorld{
    pub size: Point,
    pub tiles: Vec<Vec<_TileType>>,
    pub contents: Vec<Vec<Content>>,
    pub elevation: Vec<Vec<usize>>
}
impl PreWorld{
    pub fn new(size: Point, tiles: Vec<Vec<_TileType>>, contents: Vec<Vec<Content>>, elevation: Vec<Vec<usize>>) -> Self {
        Self { size, tiles, contents, elevation }
    }
}
impl Default for PreWorld{
    fn default() -> Self {
        Self {
            size:Point::new( 200,200),
            tiles: vec![],
            contents: vec![],
            elevation:vec![]
        }
    }
}