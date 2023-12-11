use crate::image_export::image_export::export_to_image;
use crate::world_gen::{ gen_world};
mod world_gen;

mod pathfinding;
mod zones;
mod tile;
mod image_export;
mod content_distributions;

fn main() {
    let world = gen_world(300, 300);
    export_to_image(&world, "map.png")
}
