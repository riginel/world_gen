use crate::world_gen::{export_to_image, gen_world};

mod world_gen;

mod pathfinding;

fn main() {
    let world = gen_world(200, 200);
    export_to_image(&world, "map.png")
}
