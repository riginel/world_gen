use std::collections::HashMap;
use std::ops::Range;
use robotics_lib::world::tile::{Content};
use robotics_lib::world::world_generator;
use crate::tile::_TileType;

pub struct ContentDist{
    distribution: HashMap<_TileType,HashMap<Range<usize>,Content>>
}
impl ContentDist {
    pub fn get_content(&self,tile:_TileType, value: usize)->Content{
        if let Some(x)=self.distribution.get(&tile){
            for (range,content) in x.iter(){
                if range.contains(&value){
                    return content.clone();
                }
            }
        }

        Content::None
    }
}
impl Default for ContentDist{
    fn default() -> Self {
        let map = HashMap::from(
            [
                (_TileType::Grass, HashMap::from(
                        [
                            (0..50,Content::None),
                            (50..70,Content::Tree(1)),
                            (70..73,Content::Garbage(1)),
                            (73..76,Content::Bin(0..9)),
                            (76..78,Content::Fire),
                            (78..80,Content::Coin(3)),
                            (80..85,Content::Bush(1)),
                            (85..90,Content::Rock(2)),
                            (90..92,Content::Bank(0..100)),
                            (92..95,Content::Crate(0..5)),
                            (95..100,Content::Building)
                            ]
                    )),
                (_TileType::Mountain,HashMap::from([
                    (0..60,Content::None),
                    (60..80,Content::Rock(4)),
                    (80..85,Content::Coin(5)),
                    (85..100,Content::Fire),


                ])),
                (_TileType::Water,HashMap::from([
                    (0..90,Content::Water(5)),
                    (90..300,Content::Fish(2)),

                ])),
                (_TileType::Lava,HashMap::from([
                    (0..100,Content::None)
                ])),
                (_TileType::DeepWater,HashMap::from([
                    (0..90,Content::Water(10)),
                    (90..100,Content::Fish(10))
                ])),
                (_TileType::Hill,HashMap::from([
                    (0..60,Content::None),
                    (60..65,Content::Tree(1)),
                    (65..80,Content::Market(3)),
                    (80..85,Content::Bank(0..100)),
                    (85..95,Content::Coin(10)),
                    (95..100,Content::Crate(0..5))
                ]
                )),
                (_TileType::Sand,HashMap::from([
                    (0..60,Content::None),
                    (60..70,Content::Coin(3)),
                    (70..80,Content::Rock(1)),
                    (80..90,Content::Garbage(2)),
                    (90..95,Content::Bin(0..10)),
                    (95..100,Content::Scarecrow)
                ])),
                (_TileType::Road,HashMap::from([
                    (0..80,Content::None),
                    (80..85,Content::Coin(2)),
                    (85..90,Content::Garbage(1)),
                    (90..100,Content::Rock(1))
                ]))



                ]
        );
        Self{
            distribution:map
        }
    }
}