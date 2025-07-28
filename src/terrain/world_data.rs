use crate::terrain::chunk::TerrainChunkState;
use bevy::prelude::*;
use std::collections::HashMap;

/// Global resource containing all loaded terrain data
#[derive(Resource)]
pub struct WorldData {
    /// All chunks indexed by (chunk_x, chunk_z) coordinates
    pub chunks: HashMap<(i32, i32), TerrainChunkState>,
    /// World bounds for all loaded data
    pub bounds: (i32, i32, i32, i32), // min_x, max_x, min_z, max_z
    /// Center offset for coordinate system
    pub center_offset: Vec2,
}

impl WorldData {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            bounds: (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
            center_offset: Vec2::ZERO,
        }
    }

    /// Add a region of chunks to the world data
    pub fn add_region(&mut self, chunks: Vec<TerrainChunkState>) {
        for chunk in chunks {
            // Update bounds
            self.bounds.0 = self.bounds.0.min(chunk.chunk_x);
            self.bounds.1 = self.bounds.1.max(chunk.chunk_x);
            self.bounds.2 = self.bounds.2.min(chunk.chunk_z);
            self.bounds.3 = self.bounds.3.max(chunk.chunk_z);

            // Store chunk
            self.chunks.insert((chunk.chunk_x, chunk.chunk_z), chunk);
        }
    }

    /// Calculate center offset after all regions are loaded
    pub fn finalize(&mut self) {
        if self.bounds.0 != i32::MAX {
            let center_x = (self.bounds.0 + self.bounds.1) as f32 / 2.0;
            let center_z = (self.bounds.2 + self.bounds.3) as f32 / 2.0;

            // Convert to world coordinates (assuming 32x32 chunks)
            self.center_offset = Vec2::new(center_x * 32.0 * 13.0, center_z * 32.0 * 13.0);
        }
    }

    /// Get chunk by coordinates
    pub fn get_chunk(&self, chunk_x: i32, chunk_z: i32) -> Option<&TerrainChunkState> {
        self.chunks.get(&(chunk_x, chunk_z))
    }

    /// Get chunks within a world coordinate radius
    pub fn get_chunks_in_radius(&self, center: Vec2, radius: f32) -> Vec<&TerrainChunkState> {
        let chunk_size = 32.0 * 13.0; // 32 hexes * hex size

        // Convert camera position to chunk grid coordinates
        let camera_chunk_x = ((center.x + self.center_offset.x) / chunk_size).round() as i32;
        let camera_chunk_z = ((center.y + self.center_offset.y) / chunk_size).round() as i32;

        // Calculate chunk radius from world radius
        let chunk_radius = ((radius / chunk_size) + 0.5) as i32;

        self.chunks
            .values()
            .filter(|chunk| {
                // Calculate distance in chunk grid coordinates
                let dx = (chunk.chunk_x - camera_chunk_x).abs();
                let dz = (chunk.chunk_z - camera_chunk_z).abs();

                // Use Manhattan distance for better grid coverage
                dx + dz <= chunk_radius
            })
            .collect()
    }

    /// Get chunks in a rectangular viewport area
    pub fn get_chunks_in_viewport(
        &self,
        center: Vec2,
        width: f32,
        height: f32,
    ) -> Vec<&TerrainChunkState> {
        let chunk_size = 32.0 * 13.0; // 32 hexes * hex size

        // Camera position is already in world coordinates relative to center
        // Convert camera world position to absolute world coordinates, then to chunk grid
        let camera_world_x = center.x + self.center_offset.x;
        let camera_world_z = center.y + self.center_offset.y;

        // Convert to chunk grid coordinates
        let camera_chunk_x = (camera_world_x / chunk_size).floor() as i32;
        let camera_chunk_z = (camera_world_z / chunk_size).floor() as i32;

        // Calculate how many chunks we need to cover the viewport
        let chunks_wide = (width / chunk_size).ceil() as i32;
        let chunks_tall = (height / chunk_size).ceil() as i32;

        // Add radius for smooth loading
        let radius = 3; // chunks around viewport
        let half_width = chunks_wide / 2 + radius;
        let half_height = chunks_tall / 2 + radius;

        let min_x = camera_chunk_x - half_width;
        let max_x = camera_chunk_x + half_width;
        let min_z = camera_chunk_z - half_height;
        let max_z = camera_chunk_z + half_height;

        // Debug the coordinate conversion
        log::info!(
            "Camera at ({:.1}, {:.1}) -> world ({:.1}, {:.1}) -> chunk ({}, {})",
            center.x,
            center.y,
            camera_world_x,
            camera_world_z,
            camera_chunk_x,
            camera_chunk_z
        );
        log::info!(
            "Viewport {}x{} -> chunks {}x{} + {} radius -> range X=[{}, {}], Z=[{}, {}]",
            width,
            height,
            chunks_wide,
            chunks_tall,
            radius,
            min_x,
            max_x,
            min_z,
            max_z
        );

        let selected: Vec<_> = self
            .chunks
            .values()
            .filter(|chunk| {
                chunk.chunk_x >= min_x
                    && chunk.chunk_x <= max_x
                    && chunk.chunk_z >= min_z
                    && chunk.chunk_z <= max_z
            })
            .collect();

        log::info!(
            "Selected {} chunks from bounds [{}, {}] x [{}, {}]. Available chunks: {}",
            selected.len(),
            self.bounds.0,
            self.bounds.1,
            self.bounds.2,
            self.bounds.3,
            self.chunks.len()
        );

        // Log some actual chunk coordinates that were selected
        for (i, chunk) in selected.iter().take(5).enumerate() {
            log::info!("  Chunk {}: ({}, {})", i, chunk.chunk_x, chunk.chunk_z);
        }

        selected
    }
}

impl FromWorld for WorldData {
    fn from_world(_world: &mut World) -> Self {
        WorldData::new()
    }
}
