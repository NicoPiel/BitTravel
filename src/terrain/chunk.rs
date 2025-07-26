use bevy::math::ops::sqrt;
use serde::{Deserialize, Serialize};
use spacetimedb_lib::{bsatn::Deserializer, buffer::Cursor, de::VariantAccess};
use std::{
    fs::{self, File},
    io::Read,
};

use crate::terrain::cell::Cell;

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

    pub fn from_dir(dir_path: &str) -> Result<Vec<Vec<Self>>, Box<dyn std::error::Error>> {
        let file_extension = "bsatn";

        log::info!("Reading data...");

        let chunks = fs::read_dir(dir_path)?
            .filter_map(|entry| {
                let entry = match entry {
                    Ok(e) => e,
                    Err(e) => {
                        log::warn!("Error reading directory entry: {e}");
                        return None;
                    }
                };

                let path = entry.path();
                let path_str = match path.to_str() {
                    Some(s) => s,
                    None => {
                        log::warn!("Failed to convert path to string: {path:?}");
                        return None;
                    }
                };

                if path.extension().is_some_and(|ext| ext == file_extension) {
                    match TerrainChunkState::from_bsatn(path_str) {
                        Ok(chunk) => Some(chunk),
                        Err(e) => {
                            log::warn!("Failed to load chunk from {path_str}: {e}");
                            None
                        }
                    }
                } else {
                    None
                }
            })
            .collect();

        Ok(chunks)
    }

    pub fn cells(&self) -> Vec<Cell> {
        let mut cells: Vec<Cell> = Vec::new();

        let dimension = self.biomes.len();
        let chunk_radius = sqrt(dimension as f32).floor() as i32;

        for i in 0..chunk_radius {
            for j in 0..chunk_radius {
                let cell_in_chunk = i * chunk_radius + j;
                let cell_x = chunk_radius * self.chunk_x + i;
                let cell_z = chunk_radius * self.chunk_z + j;

                cells.push(Cell {
                    cell_x,
                    cell_z,
                    biome: self.biomes[cell_in_chunk as usize],
                    elevation: self.elevations[cell_in_chunk as usize],
                });
            }
        }
        cells
    }
}
