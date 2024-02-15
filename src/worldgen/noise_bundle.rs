use std::{cmp::max, collections::HashMap, ops::Range};

use noise::{NoiseFn, Perlin};
use rand::{rngs::StdRng, seq::SliceRandom, thread_rng, Rng, SeedableRng};
use robotics_lib::world::tile::{Content, TileType};

use crate::utils::{generator_error::GeneratorError, tile::PreTile};

/// Bundle of the parameters to generate the Tile: Type/Content
/// and the "map topology"
pub  struct NoiseBundle {
    /// IMPORTANT: the seed it's NOT used for the NoiseFunction seed, just for
    /// the StdRng of the TileType/Content dispatcher
    /// If you want to seed the NoiseFunction pass the initialized noise_fn
    seed: u32,
    /// Pass already seeded. must implement the NoiseFn<f64, 2> Trait of the noise crate.
    /// PASS the noise function ALREADY INITIALIZED WITH THE SEED!!!
    pub noise_fn: Box<dyn NoiseFn<f64, 2>>,
    /// "Zoom-out" factor.
    /// The bigger the scale the bigger the zoom out of the world,
    /// also more "Chaotic" with a bigger entropy
    scale: f64,
    /// IMPORTANT: if the noise_fn is set up with the octaves,
    /// Set this paramer to ZERO 0. 
    octaves: usize,
    /// Noise distribution as promised
    noise_distribution: Vec<(Range<usize>, Vec<TileType>)>,
    /// Content distribution as promised
    content_distribution: ContentDist,
    rng: StdRng,
}

impl Default for NoiseBundle{
    fn default() -> Self {
        let rastafariani_seed = thread_rng().gen::<u32>();

        Self {
            seed: rastafariani_seed,
            noise_fn: Box::new(Perlin::new(rastafariani_seed)),
            scale: 4.20, // It's Rust time :)
            octaves: 5, 
            noise_distribution: vec![
                (0..25, vec![TileType::DeepWater, TileType::ShallowWater]),
                (25..35, vec![TileType::ShallowWater]),
                (
                    35..50,
                    vec![TileType::Sand, TileType::Grass, TileType::Grass],
                ),
                (50..60, vec![TileType::Grass]),
                (
                    60..70,
                    vec![TileType::Sand, TileType::Hill, TileType::Mountain],
                ),
                (70..81, vec![TileType::Mountain]),
                (81..100, vec![TileType::Snow, TileType::Lava]),
            ], 
            content_distribution: ContentDist::default(),
            rng: StdRng::seed_from_u64(rastafariani_seed as u64),
        }
    } 
}

////////////////////////////////////////////////////////////////////


impl  NoiseBundle {
    pub fn new()-> Self {
        Self::default()
    }

    pub fn set_seed(mut self, seed: u32)-> Self {
        self.seed = seed;
        self.rng = StdRng::seed_from_u64(seed as u64);
        self
    }

    pub fn set_noise_fn(mut self, noise_fn: Box<dyn NoiseFn<f64, 2>>) -> Self {
        self.noise_fn = noise_fn;
        self
    }

    pub fn set_scale(mut self, scale: f64)-> Self {
        self.scale = scale;
        self
    }

    pub fn set_octaves(mut self, octaves: usize)-> Self {
        self.octaves = octaves;
        self
    }

    pub fn set_noise_distribution(mut self, noise_distribution: Vec<(Range<usize>, Vec<TileType>)> )-> Self {
        if !noise_distribution.is_empty() {
            self.noise_distribution = noise_distribution;
        }
        self
    }

    pub fn set_content_distribution(mut self, content_distribution: ContentDist)-> Self {
        self.content_distribution = content_distribution;
        self
    }


    pub (crate) fn get_seed(&self)-> u32 {
        self.seed
    }

    pub (crate) fn get_scale(&self)-> f64 {
        self.scale
    }

    pub (crate) fn get_octaves(&self)-> usize {
        self.octaves
    }

    pub (crate) fn get_noise_distribution(&self)-> Vec<(Range<usize>, Vec<TileType>)> {
        self.noise_distribution.clone()
}


}

////////////////////////////////////////////////////////////////////

impl NoiseBundle {
    /// Noise -> Zone(id), Zone(id) -> TileType
    /// This function function does the id -> TileType according to the noise_distribution
    /// If something goes wrong just returns ShallowWater.
    pub(crate) fn zone_to_tiletype_dispatcher(&self, id: usize) -> TileType {
        let (_, dist) = self.noise_distribution[id].clone();
        if dist.len() == 1 {
            return dist[0];
        }
        match dist.choose(&mut thread_rng()){
            Some(tile_type) => *tile_type,
            None => TileType::ShallowWater,
        }
    }

    /// Noise -> Zone(id), Zone(id) -> TileType
    /// This function compute the Noise -> Zone(id)
    pub(crate) fn noise_to_pretile(&self, z: usize) -> PreTile {
        assert!(!self.noise_distribution.is_empty());
        let mut counter: usize = 0;
        for (range, _) in self.noise_distribution.iter() {
            if range.contains(&z) {
                return PreTile::new(counter, z);
            }
            counter += 1;
        }
        PreTile::new(max::<usize>(self.noise_distribution.len(), 1) - 1, z)
    }

    pub(crate) fn put_content(&self, ttype: TileType) -> Content {
        self.content_distribution.get_content(ttype, rand::thread_rng().gen_range(0..=100))
    }
}

pub struct ContentDist{
    dist: HashMap<TileType,Vec<(Range<usize>,Content)>>
}

impl ContentDist {
    pub fn get_content(&self, tile: TileType, value: usize) -> Content {
        if let Some(x) = self.dist.get(&tile) {
            for (range, content) in x.iter() {
                if range.contains(&value) {
                    return content.clone();
                }
            }
        }

        Content::None
    }
    pub fn new(dist: HashMap<TileType, Vec<(Range<usize>, Content)>>) -> Self {
        ContentDist { dist }
    }
    /*
    checks these conditions:
    - All TileTypes are covered
    - for each TileType, all values from 0 to 100 are covered
    - TileType and content must be compatible
    */
    pub fn is_valid(&self) -> Result<(), GeneratorError> {
        //check if the map contains all possible tiletypes
        if self.dist.len() != 11 {
            return Err(GeneratorError::NonExhaustiveContentDistribution);
        };
        //check if there's an activated teleport
        if self.dist.contains_key(&TileType::Teleport(true)) {
            return Err(GeneratorError::ActiveTeleport);
        };
        //check if ranges overlap and do not cover from 0 to 100
        for (tile, map) in self.dist.iter() {
            if map.is_empty() {
                return Err(GeneratorError::NonExhaustiveContentDistribution);
            };
            let mut covering_range: Range<usize> = 0..0;
            let tile_properties = tile.properties();
            for (range, content) in map.iter() {
                //check if tile can hold
                if !tile_properties.can_hold(content) {
                    return Err(GeneratorError::InvalidContent(*tile,content.clone()));

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
        Self { dist: map }
    }
}