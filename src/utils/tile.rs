#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PreTileType {
    Zone(usize),
    Void,
}

#[derive(Debug, Clone, Copy)]
pub struct PreTile {
    pub pre_tiletype: PreTileType,
    pub elevation: usize,
}

impl PreTile {
    pub fn new(id: usize, elevation: usize) -> Self {
        Self {
            pre_tiletype: PreTileType::Zone(id),
            elevation,
        }
    }
}