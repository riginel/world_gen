use noise::{Constant, Displace, NoiseFn, ScalePoint};
use std::collections::HashMap;
use std::mem;
use noise::*;
use rand::prelude::*;
use rand::distributions::Uniform;
use rand::seq::SliceRandom;
use robotics_lib::utils::LibError;
use robotics_lib::world::environmental_conditions::{EnvironmentalConditions, WeatherType};
use crate::pathfinding::a_star::*;
use crate::utils::tile::*;
use crate::zones::zones::Zones;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::world_generator::{Generator, World};
use crate::customization::noise_to_tile::NoiseDist;
use crate::customization::noise_to_tile::NoiseBundle;
use crate::world_builder::WorldBuilder;

impl PreWorld{
    pub fn gen_world_from_builder(builder:WorldBuilder)->Result<PreWorld,LibError>   {
        let (size,
            elevation_factor,
            noise_function,
            content_distribution,
            noise_to_tile,
            city_content_distribution,
            environmental_conditions,
            max_score,
            score_table) = builder.to_tuple();

        let (mut world, mut elevation_vec) = noise_function.generate_tiles_and_elevation(size,noise_to_tile,elevation_factor);

        let zones = Zones::get_zones(&mut world);
        let mut world:Vec<Vec<TileType>>  = world.iter().map(|vec| vec.iter().map(|t|->TileType {t.to_tiletype()}).collect()).collect();
        let mut content_vec = Vec::<Vec<Content>>::with_capacity(size);
        let mut range_generator = Uniform::new(0,100);
        for i in 0..size{
            content_vec.push(Vec::with_capacity(size));
            for j in 0..size{
                content_vec[i].push(content_distribution.get_content(world[i][j],range_generator.sample(&mut rand::thread_rng())));
            }
        }
        zones.fill_cities_with_content(&mut world, &mut content_vec,&city_content_distribution);
        let world = to_tiles_vec(world,content_vec,elevation_vec)?;
        let pre_world = PreWorld{
            size:size,
            tiles:world,
            environmental_conditions,
            max_score,
            score_table
        };
        Ok(pre_world)

    }

    pub fn save_to_file(){
        todo!()
    }
    pub fn build_from_file(){
        todo!()
    }
}

pub fn default_weather_conditions(starting_hour:u8, time_progression:u8, number:usize) ->Result<EnvironmentalConditions,LibError>{
    let weather_vec = vec![WeatherType::Sunny,WeatherType::Rainy,WeatherType::Foggy,WeatherType::TrentinoSnow,WeatherType::TropicalMonsoon];
    let mut weather_cycle:Vec<WeatherType> = Vec::new();
    for i in 0..number{
        weather_cycle.push(weather_vec.choose(&mut thread_rng()).unwrap().clone());

    }
    EnvironmentalConditions::new(weather_cycle.as_slice(),starting_hour,time_progression)
}






pub struct PreWorld{
    pub size: usize,
    pub tiles: Vec<Vec<Tile>>,
    pub environmental_conditions:EnvironmentalConditions,
    pub max_score:f32,
    pub score_table:Option<HashMap<Content,f32>>
}

impl Generator for PreWorld{
    fn gen(&mut self)->World {
        let tiles = mem::take(&mut self.tiles);
        let score_table = mem::take(&mut self.score_table);

        return (tiles,(self.size,self.size),self.environmental_conditions.clone(),self.max_score,score_table);

    }
}
