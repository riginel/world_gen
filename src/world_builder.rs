use std::collections::HashMap;


use robotics_lib::utils::LibError;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::tile::Content;
use crate::customization::content_distribution::{CityContentDist, ContentDist};
use crate::customization::noise_to_tile::{NoiseBundle, NoiseDist};
use crate::world_gen::{default_weather_conditions, PreWorld};

pub struct WorldBuilder{
    size:usize,
    elevation_factor:f64,
    noise_function:NoiseBundle,
    content_distribution:ContentDist,
    noise_to_tile:NoiseDist,
    city_content_distribution:CityContentDist,
    environmental_conditions: EnvironmentalConditions,
    max_score:f32,
    score_table: Option<HashMap<Content,f32>>
    
}
impl Default for WorldBuilder{
    fn default() -> Self {
        Self {

            size: 200,
            elevation_factor: 1.0,
            noise_function: NoiseBundle::default(),
            content_distribution: ContentDist::default(),
            noise_to_tile: NoiseDist::default(),
            city_content_distribution: CityContentDist::default(),
            environmental_conditions:default_weather_conditions(5,20,10).unwrap(),
            max_score:1000.0,
            score_table:None
        }
    }
}
//WorldBuilder uses the builder pattern like in this example
//https://rust-unofficial.github.io/patterns/patterns/creational/builder.html
impl WorldBuilder{
    pub fn new()->Self{
        Self::default()
    }
    pub fn set_size(mut self, size: usize)->Self {
        self.size = size;
        self
    }
    pub fn set_elevation_factor(mut self, elevation_factor: f64)->Self {
        self.elevation_factor = elevation_factor;
        self
    }
    pub fn set_noise_function(mut self, noise_function:NoiseBundle)->Self {
        self.noise_function = noise_function;
        self
    }

    pub fn set_content_distribution(mut self, content_distribution: ContentDist)->Self {
        self.content_distribution = content_distribution;
        self
    }
    pub fn set_noise_to_tile(mut self, noise_to_tile: NoiseDist)->Self {
        self.noise_to_tile = noise_to_tile;
        self
    }
    pub fn set_city_content_distribution(mut self, city_content_distribution: CityContentDist) ->Self{
        self.city_content_distribution = city_content_distribution;
        self
    }
    pub fn set_weather_condition(mut self, env_cond:EnvironmentalConditions)->Self{
        self.environmental_conditions = env_cond;
        self
    }
    pub fn set_max_score(mut self, max_score:f32)-> Self{
        self.max_score = max_score;
        self
    }
    pub fn set_score_table(mut self, score_table: HashMap<Content,f32>)->Self{
        self.score_table = Some(score_table);
        self
    }
    pub fn build(self)->Result<PreWorld,LibError>{
        PreWorld::gen_world_from_builder(self)
    }


    pub fn to_tuple(self)->(usize,f64,NoiseBundle,ContentDist,NoiseDist,CityContentDist,EnvironmentalConditions,f32,Option<HashMap<Content,f32>>) {
        (self.size,self.elevation_factor,self.noise_function,self.content_distribution,self.noise_to_tile,self.city_content_distribution,self.environmental_conditions,self.max_score,self.score_table)
    }
}
