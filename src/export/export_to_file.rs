use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use serde_json::Error;
use crate::world_gen::PreWorld;

pub fn export_to_file(world: &PreWorld, file_path: String)->Result<(),()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(writer, world)?;
    writer.flush()?;
    Ok(())
}
pub fn import_from_file(file_path: String)->Result<PreWorld,()>{
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let world:serde_json::Result<PreWorld>  =serde_json::from_reader(reader);
    world.map_err(|e| ())
}
