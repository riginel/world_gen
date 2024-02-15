

use robotics_lib::world::world_generator::Generator;

use crate::{export::export_to_file::export_to_file, worldgen::{generator::WorldGenerator, noise_bundle::NoiseBundle}};

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