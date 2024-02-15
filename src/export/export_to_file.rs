use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use robotics_lib::world::tile::Tile;

pub (crate) fn export_to_file(map: &robotics_lib::world::world_generator::World, file_path: String) -> Result<(), String> {
    let file = File::create(file_path).map_err(|_| String::from("Can't open file"))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, map).map_err(|_| String::from("Serialization failed"))?;
    writer.flush().map_err(|_| String::from("Serialization failed"))?;
    Ok(())
}

pub (crate) fn import_from_file(file_path: String) -> Result<robotics_lib::world::world_generator::World, String> {
    let file = File::open(file_path).map_err(|_| String::from("Can't open file"))?;
    let reader = BufReader::new(file);
    let world: serde_json::Result<robotics_lib::world::world_generator::World> = serde_json::from_reader(reader);
    world.map_err(|_| (String::from("Deserialization failed!")))
}