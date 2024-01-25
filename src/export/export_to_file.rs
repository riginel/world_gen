use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use crate::utils::generator_error::GeneratorError;
use crate::world_gen::PreWorld;
impl PreWorld {
    pub fn export_to_file(&self, file_path: String) -> Result<(), GeneratorError> {
        let file = File::create(file_path).map_err(|_|GeneratorError::FileExportError("Can't open file".into()))?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self).map_err(|_| GeneratorError::FileExportError(String::from("Serialization failed")))?;
        writer.flush().map_err(|_| GeneratorError::FileExportError(String::from("Serialization failed")))?;
        Ok(())
    }
    pub fn import_from_file(file_path: String) -> Result<Self, GeneratorError> {
        let file = File::open(file_path).map_err(|_| GeneratorError::FileImportError(String::from("Can't open file")))?;
        let reader = BufReader::new(file);
        let world: serde_json::Result<PreWorld> = serde_json::from_reader(reader);
        world.map_err(|_| (GeneratorError::FileImportError(String::from("Deserialization failed!"))))
    }
}