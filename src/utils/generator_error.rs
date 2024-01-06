use robotics_lib::world::tile::{Content, TileType};

#[derive(Debug,Eq,PartialEq)]
pub enum GeneratorError{
    InvalidWorldSize,
    InvalidContent(TileType,Content),
    NonExhaustiveContentDistribution,
    OverlappingDistribution,
    ActiveTeleport,
}