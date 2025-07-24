use std::collections::hash_set::HashSet;
use std::fs;

use itertools::Itertools;

use crate::chunk::TerrainChunkState;

mod chunk;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let regions = match read_dir() {
        Ok(r) => {
            log::info!("Done reading data");
            r
        }
        Err(_) => panic!("Regions couldn't be deserialised!"),
    };

    let mut region_index = 1;

    for region in &regions {
        let chunks = &region.len();
        let cells = &region.iter().fold(0, |acc, r| acc + r.biomes.len());

        let ending_x = &region
            .iter()
            .max_by(|r1, r2| r1.chunk_x.cmp(&r2.chunk_x))
            .unwrap()
            .chunk_x;

        let ending_y = &region
            .iter()
            .max_by(|r1, r2| r1.chunk_z.cmp(&r2.chunk_z))
            .unwrap()
            .chunk_z;

        let caves = &region.iter().fold(0, |acc, r| {
            if r.chunk_x == 0 && r.chunk_z == 0 {
                acc + 1
            } else {
                acc
            }
        });

        let min_density = &region
            .iter()
            .flat_map(|r| r.biome_density.iter())
            .min_by(|x, y| x.cmp(y))
            .unwrap();

        let max_density = &region
            .iter()
            .flat_map(|r| r.biome_density.iter())
            .max_by(|x, y| x.cmp(y))
            .unwrap();

        let unique_densities = &region
            .iter()
            .flat_map(|r| r.biome_density.iter())
            .unique()
            .collect_vec();

        log::info!("Region: {region_index}");
        log::info!("Chunks: {chunks}, cells: {cells}");
        log::info!("Ending chunk coords: [{ending_x}:{ending_y}]");
        log::info!("Caves? {caves}");
        log::info!("Chunks without instances {:?}", chunks - caves);
        log::info!("Unique densities {:?}", unique_densities);
        log::info!("----------------------------");

        region_index += 1;
    }

    let total_chunks = &regions.iter().fold(0, |acc, r| acc + r.len());

    log::info!("Chunks in total: {total_chunks}",);
}

fn read_dir() -> Result<Vec<Vec<TerrainChunkState>>, Box<dyn std::error::Error>> {
    let dir_path = "./data";
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
                    log::warn!("Failed to convert path to string: {:?}", path);
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
