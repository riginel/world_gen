use robotics_lib::world::world_generator::Generator;

use crate::image_export::image_export::export_to_image;
use crate::world_builder::WorldBuilder;

mod world_gen;

mod pathfinding;
mod zones;
mod image_export;
mod customization;
mod world_builder;
mod utils;

fn main() {

    let mut world= WorldBuilder::new().build();
    match world{
        Ok(mut preWorld) => {export_to_image(&preWorld,"map.png");
            preWorld.gen();}
        Err(e) => {println!("{:?}",e);}
    };

}
