mod chunk;

#[tokio::main]
async fn main() {
    chunk::TerrainChunkState::from_file(1).ok();
}
