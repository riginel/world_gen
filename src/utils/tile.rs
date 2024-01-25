use robotics_lib::utils::LibError;
use robotics_lib::world::tile::{Content, Tile, TileType};
use crate::pathfinding::a_star::Walkable;

#[derive(Debug, Copy, Clone,PartialEq,Eq,Hash)]
pub enum PreTileType {
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

impl PreTileType {
    pub fn to_tiletype(&self)->TileType{
        match self{
            PreTileType::Grass => {TileType::Grass}
            PreTileType::Water => {TileType::ShallowWater}
            PreTileType::DeepWater => {TileType::DeepWater}
            PreTileType::Sand => {TileType::Sand}
            PreTileType::Mountain => {TileType::Mountain}
            PreTileType::Lava => {TileType::Lava}
            PreTileType::Road => {TileType::Street}
            PreTileType::Hill => {TileType::Hill}
            _ =>{TileType::Grass}
        }
    }
}
//converts vectors of TileType,Content, and Elevation to a vector of Tile
pub fn to_tiles_vec(tiletypes:Vec<Vec<TileType>>,contents:Vec<Vec<Content>>,elevation:Vec<Vec<usize>>)->Result<Vec<Vec<Tile>>,LibError>{
    if tiletypes.len() != contents.len() || tiletypes.len() != elevation.len() {
        return Err(LibError::OutOfBounds);
    }
    let size = tiletypes.len();
    let mut tile_vec: Vec<Vec<Tile>> = Vec::with_capacity(size);
    for i in 0.. size{
        let mut row:Vec<Tile> = Vec::with_capacity(size);
        for j in 0..size{
            let tile = Tile{
                tile_type: tiletypes[i][j].clone(),
                content: contents[i][j].clone(),
                elevation: elevation[i][j]
            };
            row.push(tile);
        }
        tile_vec.push(row)
    }
    Ok(tile_vec)
}
impl Walkable for PreTileType {
    fn cost(&self) -> u32 {
        match self{
            PreTileType::Grass => {1}
            PreTileType::Water => {3}
            PreTileType::DeepWater => {10}
            PreTileType::Sand => {1}
            PreTileType::Mountain => {500}
            PreTileType::Lava => {100000}
            PreTileType::Road => {0}
            _ => {1}
        }
    }
}
