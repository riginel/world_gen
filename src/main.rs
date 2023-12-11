use crate::world_gen::{export_to_image, gen_world};

mod world_gen;

mod pathfinding;
mod zones;
mod tile;
mod image_export;

fn main() {
    let world = gen_world(300, 300);
    export_to_image(&world, "map.png")
}
