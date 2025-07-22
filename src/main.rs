use std::fs;

use crate::chunk::TerrainChunkState;

mod chunk;

#[tokio::main]
async fn main() {
    let regions = match read_dir() {
        Ok(r) => r,
        Err(_) => panic!("Regions couldn't be deserialised!"),
    };

    println!("BSATN: {:?}", regions[0].len());
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
