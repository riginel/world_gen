use export::{export_to_file::{export_to_file, import_from_file}, export_to_image::export_to_image};
use robotics_lib::{event::events::Event, world::world_generator::Generator};
use worldgen::{generator::WorldGenerator, noise_bundle::{NoiseBundle}};

pub mod utils;
pub mod export;
pub mod worldgen;

#[cfg(test)]
mod tests;


#[test]
fn test(){
    let noise_bundle = NoiseBundle::new()
        .set_seed(42069420)
        .set_octaves(5)
        .set_scale(4.2069);


    let mut generator = WorldGenerator::new()
        .set_size(1000)
        .set_elevation_factor(4.2)
        .set_max_score(420.0)
        .set_noise_bundle(noise_bundle);


    let world = generator.gen();
    assert!(export_to_image("mappamondo.png", &world.0).is_ok());
    assert!(export_to_file(&world, "mappamondo".to_string()).is_ok());
    let imported_world = import_from_file("mappamondo".to_string()).unwrap();
    assert!(export_to_image("mappamondo.png", &imported_world.0).is_ok());
    //println!("{:?}", world.0);
}