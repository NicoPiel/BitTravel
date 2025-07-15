use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TerrainChunkState {
    pub chunk_index: u64,
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub dimension: u32,
    pub biomes: Vec<u32>,
    pub biome_density: Vec<u32>,
    pub elevations: Vec<i16>,
    pub water_levels: Vec<i16>,
    pub water_body_types: Vec<u8>,
    pub zoning_types: Vec<u8>,
    pub original_elevations: Vec<i16>,
}

impl TerrainChunkState {
    pub fn from_file(region: u8) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let file_path = format!("./data/terrain_chunk_state_{region}.json");

        let file = File::open(&file_path).map_err(|e| {
            eprintln!("Error opening file '{file_path}': {e}");
            e
        })?;

        let reader = BufReader::new(file);

        let chunks: Vec<TerrainChunkState> = serde_json::from_reader(reader).map_err(|e| {
            eprintln!("Error deserializing JSON from '{file_path}': {e}");
            e
        })?;

        if !chunks.is_empty() {
            println!(
                "Successfully loaded terrain chunks from '{}'. First chunk index: {}",
                file_path, chunks[0].chunk_index
            );
        } else {
            println!(
                "Successfully loaded terrain chunks from '{file_path}', but the file was empty or contained an empty array."
            );
        }

        Ok(chunks)
    }
}
