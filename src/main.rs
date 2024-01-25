use robotics_lib::world::world_generator::Generator;


use crate::world_builder::WorldBuilder;

pub mod world_gen;
mod pathfinding;
pub mod zones;
pub mod customization;
pub mod world_builder;
pub mod utils;
pub mod export;

fn main() {

    let  world= WorldBuilder::new().build();
    println!("ciaoo");
    match world{
        Ok(mut pre_world) => {
            pre_world.export_to_image("map.png").expect("AOOO");
            pre_world.gen();}
        Err(e) => {println!("{:?}",e);}
    };

}
