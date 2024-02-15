use std::{collections::HashMap};

use rand::{seq::SliceRandom, thread_rng};
use robotics_lib::world::{environmental_conditions::{EnvironmentalConditions, WeatherType}, tile::{Content, Tile, TileType}, world_generator::Generator};

use crate::utils::{pathfinding::{build_road, shortest_path}, tile::{PreTile, PreTileType}, vector2::Vector2, zone::Zone};

use super::noise_bundle::NoiseBundle;

/// Used to generate the World, implements the 'Generator' Trait.
pub struct WorldGenerator {
    /// World size (size x size) of the map 
    size: usize,
    /// Elevation of tiles in range: [0, elevation_factor as usize]
    /// (normalized_noise([0.0, 1.0]) * elevation_factor) as usize
    elevation_factor: f64,
    /// Noise Bundle
    noise_bundle: NoiseBundle,
    score_table: Option<HashMap<Content, f32>>,
    max_score: f32,
    /// Tiles where Robot will not spawn
    /// The streets will connect every homogeneous zone EXCEPT 
    /// for the ones in not_spawnable
    not_spawnable: Vec::<TileType>,
}

impl Default for WorldGenerator {
    fn default() -> Self {
        Self { 
            size: 1024, 
            elevation_factor: 1.0,
            noise_bundle: NoiseBundle::default(),
            score_table: None, 
            max_score: 420.0, 
            not_spawnable: [TileType::Lava, TileType::DeepWater].to_vec(),
        }
    }
}

////////////////////////////////////////////////////////////////////

impl  WorldGenerator {

    /// Return a WorldGenerator::default()
    pub  fn new() -> Self {
        Self::default()
    }
    
    /// Set self.size: usize
    pub  fn set_size(mut self, size: usize) -> Self {
        
        self.size = size;
        self
    }

    /// get self.size: usize
    pub  fn get_size(self) -> usize {
        self.size
    }

    pub  fn set_elevation_factor(mut self, elevation_factor: f64) -> Self {
        self.elevation_factor = elevation_factor;
        self
    }
    
    pub  fn set_noise_bundle(mut self, noise_bundle: NoiseBundle) -> Self {
        self.noise_bundle = noise_bundle;
        self
    }

    pub  fn set_score_table(mut self, score_table: Option<HashMap<Content, f32>>) -> Self {
        self.score_table = score_table;
        self
    }
    
    pub fn set_max_score(mut self, max_score: f32) -> Self {
        self.max_score = max_score;
        self
    }

    pub fn set_notspawnable(mut self, notspawnable: Vec::<TileType>) -> Self {
        self.not_spawnable = notspawnable;
        self
    }
}

////////////////////////////////////////////////////////////////////

impl WorldGenerator {
    /// Internal computation of WorldGenerator::gen()
    fn build(&self) -> (
        Vec<Vec<Tile>>,
        (usize, usize),
        EnvironmentalConditions,
        f32,
        Option<HashMap<Content, f32>>,
    ) {
        // Assume create a basic map with only Grass and no content.
        // Used for a "fault-tolerant" approach
        let default_tile = Tile {
            tile_type:TileType::Teleport(false), 
            content: Content::None, elevation: 0};

        let mut map: Vec::<Vec::<Tile>> = vec![vec![default_tile; self.size]; self.size];
        
        // Pretile: Zone(id) 
        // Matrix of Pretile based on NoiseBundle.tile_distribution
        // Editing the tile distribution should be approached carefully.
        let mut preworld = self.generate_preworld();

        // Vector of <Zone>, 
        let zones = self.get_zones(&mut preworld);
        let mut centroids = Vec::<Vector2>::new();

        for zone in 0..zones.len() {
            for i in &zones[zone] {
                // Generate the TileType for the entire contiguous zone
                // Following the rules in tile_distribution
                let ttype: TileType = self.noise_bundle.zone_to_tiletype_dispatcher(zone);

                for j in &i.inner {
                    // Set the Tiletype for every element in the zone
                    map[j.x][j.y].tile_type = ttype;
                    // Generate the content following the rules of NoiseBundle.content_distribution
                    map[j.x][j.y].content = self.noise_bundle.put_content(ttype);
                    // (e in [0.0, 1.0] * f in [0.0, elevation_factor]) as usize
                    map[j.x][j.y].elevation = ((preworld[j.x][j.y].elevation as f64 / 100.0) * self.elevation_factor) as usize;
                }

                // Avoid to connect lavapool and deepwater with the other zones of the map
                if !self.not_spawnable.contains(&map[i.centroid.x][i.centroid.y].tile_type){
                    centroids.push(i.centroid);
                }
            }

        }

        // Implementation constrain, to "optimize" the A* search connecting the two closer zones
        if !centroids.is_empty() {
            for i in 0..centroids.len()-1{
                let  (mut min_index,mut min_distance) = (i+1,centroids[i+1].manhattan_distance(centroids[i]) );
                for j in i+1..centroids.len(){
                    let this_distance = centroids[j].manhattan_distance(centroids[i]);
                    if this_distance  <= min_distance{
                        min_distance = this_distance;
                        min_index = j;
                    }
                }
                centroids.swap(i+1, min_index);
                //println!("Building road: {}", i as f64 / centroids.len() as f64 * 100.0);
                let path = shortest_path(&map, centroids[i],centroids[i+1]);
                match path {
                    Ok(tiles) => {build_road(&mut map, tiles)},
                    Err(_) => {},
                }
            
            }
        }

        // Return "The World"
        (map, (0,0), self.default_weather_conditions( 5, 10, 25), self.max_score, self.score_table.clone())

    }

    
}

////////////////////////////////////////////////////////////////////

impl WorldGenerator {

    fn generate_preworld(&self) -> Vec<Vec<PreTile>> {
        let scale = self.noise_bundle.get_scale();
        let noise_fn = &self.noise_bundle.noise_fn;

        let mut matrix: Vec<Vec<PreTile>> = Vec::with_capacity(self.size);

        for x in 0..self.size {
            let nx = x as f64 * scale / self.size as f64;
            let mut row: Vec<PreTile> = Vec::with_capacity(self.size);
            for y in 0..self.size {
                let ny = y as f64 * scale / self.size as f64;
                let mut elevation = noise_fn.get([nx, ny]);
                
                // If the noise function wasn't set up properly to generate the result with the
                // octaves is not a problems, can be do setting NoiseBundle.octaves.
                // If the NoiseBundle.noise_fn comprehend already the octaves just set the 
                // NoiseBundle.octaves to 0.
                // IMPORTANT, if you don't follow this the side effect will be:
                // (n) x (m) octaves obtained. 
                for i in 1..self.noise_bundle.get_octaves() as i32 {
                    elevation += (1.0 / 2.0_f64.powi(i))
                        * noise_fn.get([nx * 2.0_f64.powi(i) + 0.420, ny * 2.0_f64.powi(i) - 0.420]);
                }
                // Why this magic number?
                // lim n->+inf of the series 1/(2^{n}) = 2
                // If e > 1 the TileType is not a problem, it's implemented in the dispatcher
                elevation /= 1.8765420;

                // Normalize the elevation, users already knows that they have to
                // use a noise_function that return values in [-1.0, 1.0] or wrap another
                // function with a different range 
                elevation = (elevation + 1.0) / 2.0;

                // We are assuming elevation <= 1, but if this is not respected it's not a problem.
                row.push(self.noise_bundle.noise_to_pretile((elevation * 100.0) as usize));
            }
            matrix.push(row);
        }
        matrix
    }

    pub(crate) fn get_zones(&self, preworld: &mut Vec<Vec<PreTile>>) -> Vec<Vec<Zone>> {

        let dist_size = self.noise_bundle.get_noise_distribution().len();
        let mut zones: Vec<Vec<Zone>> = Vec::with_capacity(dist_size);
        for _ in 0..dist_size {
            zones.push(vec![]);
        }

        // DFS to obtain a Vector of zones with the same TileType
        for x in 0..preworld.len() {
            for y in 0..preworld[0].len() {
                match preworld[x][y].pre_tiletype {
                    PreTileType::Zone(i) => {
                        zones[i].push(Zone::dfs(preworld, i, Vector2::new(x, y)))
                    }
                    _ => {}
                }
            }
        }
        zones
    }


    // Generate standard "weather forecast", not based on any real world dynamic.
    fn default_weather_conditions(&self, starting_hour:u8, time_progression:u8, number:usize) -> EnvironmentalConditions{
        let weather_vec = [WeatherType::Sunny,WeatherType::Rainy,WeatherType::Foggy,WeatherType::TrentinoSnow,WeatherType::TropicalMonsoon];
        let mut weather_cycle:Vec<WeatherType> = Vec::new();
        for _ in 0..number{
            weather_cycle.push(*weather_vec.choose(&mut thread_rng()).unwrap());
    
        }
        EnvironmentalConditions::new(weather_cycle.as_slice(),starting_hour,time_progression).expect("Weird")
    }
}

////////////////////////////////////////////////////////////////////


impl Generator for WorldGenerator {
    fn gen(&mut self) -> robotics_lib::world::world_generator::World {
        self.build()
    }   
}