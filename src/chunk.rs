use serde::{Deserialize, Serialize};
use spacetimedb_lib::{bsatn::Deserializer, buffer::Cursor, de::VariantAccess};
use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Serialize, spacetimedb_lib::de::Deserialize, Deserialize, Clone, PartialEq, Debug)]
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
    pub fn from_bsatn(path: &str) -> Result<Vec<Self>, Box<dyn std::error::Error>>
    where
        Self: for<'de> spacetimedb_lib::de::Deserialize<'de>,
    {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let cursor = Cursor::new(buffer);
        let mut reader = &cursor;
        let dse = Deserializer::new(&mut reader);

        let values: Vec<Self> = dse.deserialize()?;
        Ok(values)
    }
}
