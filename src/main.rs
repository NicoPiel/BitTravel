mod chunk;

#[tokio::main]
async fn main() {
    let chunks = chunk::TerrainChunkState::from_bsatn("./data/terrain_chunk_state_9.bsatn")
        .ok()
        .unwrap();

    println!("Worked? {:#?}", chunks[0]);
}
