use crate::utils::generator_error::GeneratorError;
use crate::utils::tile::PreTileType;
use robotics_lib::world::tile::Content;
use robotics_lib::world::tile::TileType;
use robotics_lib::world::tile::TileType::Teleport;
use robotics_lib::world::world_generator;
use std::collections::HashMap;
use std::ops::{Range, RangeBounds};

pub struct ContentDist {
    distribution: HashMap<TileType, Vec<(Range<usize>, Content)>>,
}
impl ContentDist {
    pub fn get_content(&self, tile: TileType, value: usize) -> Content {
        if let Some(x) = self.distribution.get(&tile) {
            for (range, content) in x.iter() {
                if range.contains(&value) {
                    return content.clone();
                }
            }
        }

        Content::None
    }
    pub fn new(distribution: HashMap<TileType, Vec<(Range<usize>, Content)>>) -> Self {
        ContentDist { distribution }
    }
    /*
    checks these conditions:
    - All TileTypes are covered
    - for each TileType, all values from 0 to 100 are covered
    - TileType and content must be compatible
    */
    pub fn is_valid(&self) -> Result<(), GeneratorError> {
        //check if the map contains all possible tiletypes
        println!("{}",self.distribution.len());
        if self.distribution.len() != 11 {
            return Err(GeneratorError::NonExhaustiveContentDistribution);
        };
        //check if there's an activated teleport
        if self.distribution.contains_key(&Teleport(true)) {
            return Err(GeneratorError::ActiveTeleport);
        };
        //check if ranges overlap and do not cover from 0 to 100
        for (tile, map) in self.distribution.iter() {
            if map.len() == 0 {
                return Err(GeneratorError::NonExhaustiveContentDistribution);
            };
            let mut covering_range: Range<usize> = 0..0;
            let tile_properties = tile.properties();
            for (range, content) in map.iter() {
                //check if tile can hold
                if !tile_properties.can_hold(content) {
                    return Err(GeneratorError::InvalidContent(tile.clone(),content.clone()));
                }
                if covering_range.start == covering_range.end {
                    covering_range = range.clone();
                    continue;
                }
                if covering_range.start >= range.start || covering_range.end != range.start {
                    return Err(GeneratorError::OverlappingDistribution);
                }

                //checks if range overlaps and if it's not in order

                covering_range = covering_range.start..range.end
            }
            if !(covering_range.start == 0 && covering_range.end == 100) {
                return Err(GeneratorError::NonExhaustiveContentDistribution);
            };
        }

        Ok(())
    }
}
impl Default for ContentDist {
    fn default() -> Self {
        let map = HashMap::from([
            (
                TileType::Grass,
                vec![
                    (0..50, Content::None),
                    (50..70, Content::Tree(1)),
                    (70..73, Content::Garbage(1)),
                    (73..76, Content::Bin(0..9)),
                    (76..78, Content::Fire),
                    (78..80, Content::Coin(3)),
                    (80..85, Content::Bush(1)),
                    (85..90, Content::Rock(2)),
                    (90..92, Content::Bank(0..100)),
                    (92..95, Content::Crate(0..5)),
                    (95..100, Content::Building),
                ],
            ),
            (
                TileType::Mountain,
                vec![
                    (0..60, Content::None),
                    (60..80, Content::Rock(4)),
                    (80..85, Content::Coin(5)),
                    (85..90, Content::Crate(0..5)),
                    (90..100,Content::Tree(1))
                ],
            ),
            (
                TileType::ShallowWater,
                vec![(0..90, Content::Water(5)), (90..300, Content::Fish(2))],
            ),
            (TileType::Lava, vec![(0..100, Content::None)]),
            (
                TileType::DeepWater,
                vec![(0..90, Content::Water(10)), (90..100, Content::Fish(10))],
            ),
            (
                TileType::Hill,
                vec![
                    (0..60, Content::None),
                    (60..65, Content::Tree(1)),
                    (65..70,Content::Garbage(2)),
                    (70..80,Content::Bin(0..5)),
                    (80..85,Content::Scarecrow),
                    (85..95, Content::Coin(10)),
                    (95..100, Content::Crate(0..5)),
                ],
            ),
            (
                TileType::Sand,
                vec![
                    (0..60, Content::None),
                    (60..70, Content::Coin(3)),
                    (70..80, Content::Rock(1)),
                    (80..90, Content::Garbage(2)),
                    (90..95, Content::Bin(0..10)),
                    (95..100, Content::Scarecrow),
                ],
            ),
            (
                TileType::Street,
                vec![
                    (0..80, Content::None),
                    (80..85, Content::Coin(2)),
                    (85..90, Content::Garbage(1)),
                    (90..100, Content::Rock(1)),
                ],
            ),
            (
                TileType::Snow,
                vec![
                    (0..90, Content::None),
                    (90..95, Content::Coin(1)),
                    (95..100, Content::Rock(3)),

                ],
            ),
            (TileType::Wall,
             vec![(0..100, Content::None)]
            ),
            (TileType::Teleport(false),
             vec![(0..100, Content::None)]
            )
        ]);
        Self { distribution: map }
    }
}

pub struct CityContentDist{
    dist: Vec<(Range<usize>,Content)>
}
impl Default for CityContentDist{
    fn default() -> Self {
        let dist:Vec<(Range<usize>,Content)> =vec![
            (0..60,Content::None),
            (60..65,Content::Market(2)),
            (65..70,Content::Bank(0..100)),
            (70..75,Content::Coin(10)),
            (75..80,Content::Crate(0..100)),
            (80..85,Content::Garbage(1)),
            (85..90,Content::Tree(1)),
            (90..100,Content::Building)
        ];
    Self{
        dist
    }
    }


}
impl CityContentDist{
    pub fn new(dist: Vec<(Range<usize>,Content)>) -> Self{
        Self{
            dist
        }
    }
    pub fn get_content(&self, value:usize)-> Content{
        for (range,content) in self.dist.iter(){
            if range.contains(&value){
                return content.clone();
            }
        }
        Content::None
    }
}
fn test() {}
#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::generator_error::*;
    #[test]
    fn validity_test() {
        let mut dist = ContentDist::default();
        println!("{:?}", dist.is_valid());
        assert!(dist.is_valid().is_ok());
    }
}
