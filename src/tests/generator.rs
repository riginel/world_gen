use std::collections::{HashMap, HashSet};
use std::ptr::hash;
use rand::Rng;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::world_generator::Generator;

use crate::{export::export_to_file::export_to_file, worldgen::{generator::WorldGenerator, noise_bundle::NoiseBundle}};
use crate::worldgen::noise_bundle::ContentDist;

use super::*;

#[test]
fn set_seed() -> (){
    let noise_bundle = NoiseBundle::new()
    .set_seed(42);
    assert_eq!(noise_bundle.get_seed(), 42);
}

#[test]
fn set_octaves() -> (){
    let noise_bundle = NoiseBundle::new()
    .set_seed(42)
    .set_octaves(7);
    assert_eq!(noise_bundle.get_octaves(), 7);
}
 
#[test]
fn not_jagged() -> () {
    let size = 420;
    let mut generator = WorldGenerator::new().set_size(size);
    let map = generator.gen().0;
    assert_eq!(map.len(), size);
    for i in map {
        assert_eq!(i.len(), size);
    }
}

// #[test]
// fn import_export() -> () {
//     let size = 100;
//     let mut generator = WorldGenerator::new().set_size(size);
//     let mut map = generator.gen().0;
//     assert_eq!(map.len(), size);
//     export_to_file(&mut map, "testa".to_string()).unwrap();
//     let tst = crate::export::export_to_file::import_from_file("testa".to_string()).unwrap();
//     assert_eq!(map, tst);

// }

#[test]
fn no_true_teleporters() {
    let mut generator = WorldGenerator::new().set_size(420);
    let map = generator.gen().0;

    assert!( !map.iter().flatten().any(|x| x.tile_type == TileType::Teleport(true)));
}
#[test]
fn the_world_is_square(){
    let rand = rand::thread_rng().gen_range(120..800);
    let mut generator = WorldGenerator::new().set_size(rand);
    let map = generator.gen().0;
    assert_eq!(map.len() ,map[0].len());
}
#[test]
fn default_content_dist_is_valid(){
    let content_dist = ContentDist::default();
    assert_eq!(content_dist.is_valid(),Ok(()))
}

#[test]
fn content_dist_is_respected(){
    let mut generator = WorldGenerator::new().set_size(420);
    let map = generator.gen().0;
    // I collect all the contents on the world map into sets for each tiletype
    let mut  actual_dist: HashMap<TileType,HashSet<Content>> = HashMap::new();

    for i in map.iter().flatten(){
        let (tile_type, content) = (i.tile_type.clone(),i.content.clone());
        actual_dist.entry(tile_type).and_modify(|x| {x.insert(content.to_default());}).or_insert(HashSet::from([content.to_default()]));
    }
    let default_content_dist = ContentDist::default();
    for (tile_type,content_set) in actual_dist{
        let content_vector = default_content_dist.dist.get(&tile_type).unwrap();
        let distribution_content_set: HashSet<Content>= content_vector.iter().map(|(_,x)| x.to_default()).collect();
        println!("first hashset: {:?}, second hashset: {:?}", distribution_content_set, content_set);
        assert!(content_set.is_subset(&distribution_content_set));
    }

}