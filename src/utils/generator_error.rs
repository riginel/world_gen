use robotics_lib::world::tile::{Content, TileType};

#[derive(Debug,Eq,PartialEq)]
pub enum GeneratorError{
    InvalidWorldSize,
    InvalidContent(TileType,Content),
    MaxContent(Content),
    NonExhaustiveContentDistribution,
    OverlappingDistribution,
    ActiveTeleport,
    ImageExportError(String),
    FileExportError(String),
    FileImportError(String)
}