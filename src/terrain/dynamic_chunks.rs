use bevy::prelude::*;
use hexx::HexLayout;
use std::collections::HashSet;

use crate::terrain::{
    camera_culling::{ChunkBounds, VisibleChunk},
    chunk_mesh::create_chunk_mesh,
    world_data::WorldData,
};

/// Component to mark dynamically spawned chunk entities
#[derive(Component)]
pub struct DynamicChunk {
    pub chunk_coords: (i32, i32),
}

/// Resource to track which chunks are currently spawned
#[derive(Resource, Default)]
pub struct SpawnedChunks {
    pub chunks: HashSet<(i32, i32)>,
    pub last_camera_pos: Vec2,
    pub last_zoom_scale: f32,
}

/// System that dynamically spawns/despawns chunks based on camera viewport
pub fn update_dynamic_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    world_data: Res<WorldData>,
    mut spawned_chunks: ResMut<SpawnedChunks>,
    camera_query: Query<(&Transform, &Projection), With<Camera>>,
    chunk_query: Query<(Entity, &DynamicChunk)>,
) {
    let Ok((camera_transform, projection)) = camera_query.single() else {
        return;
    };

    let camera_pos = camera_transform.translation.truncate();

    // Get current zoom and calculate actual viewport bounds
    let current_zoom = if let Projection::Orthographic(ref ortho) = *projection {
        ortho.scale
    } else {
        1.0
    };

    // Check if camera moved OR zoom changed significantly
    let movement_threshold = 100.0; // Prevent flickering - ~25% of chunk size
    let zoom_threshold = 0.05; // Less sensitive to zoom changes

    // Handle first run - initialize tracking values
    let is_first_run =
        spawned_chunks.last_camera_pos == Vec2::ZERO && spawned_chunks.last_zoom_scale == 0.0;

    let camera_moved = if is_first_run {
        true // Force update on first run
    } else {
        spawned_chunks.last_camera_pos.distance(camera_pos) > movement_threshold
    };

    let zoom_changed = if is_first_run {
        false // Don't double-trigger on first run
    } else {
        (spawned_chunks.last_zoom_scale - current_zoom).abs() > zoom_threshold
    };

    if !camera_moved && !zoom_changed {
        return;
    }

    // Update tracking values
    spawned_chunks.last_camera_pos = camera_pos;
    spawned_chunks.last_zoom_scale = current_zoom;

    // Calculate viewport bounds in world coordinates
    let window_size = 1000.0; // Window is 1000x1000

    // Balanced scale factor for proper viewport coverage without memory explosion
    let scale_factor = 25.0; // Target ~400-800 chunks for good coverage
    let world_viewport_half_width = window_size * current_zoom * scale_factor * 0.5;
    let world_viewport_half_height = window_size * current_zoom * scale_factor * 0.5;

    log::info!(
        "Using scale factor: {} for balanced viewport coverage",
        scale_factor
    );

    // Camera viewport bounds in world coordinates
    let viewport_min_x = camera_pos.x - world_viewport_half_width;
    let viewport_max_x = camera_pos.x + world_viewport_half_width;
    let viewport_min_y = camera_pos.y - world_viewport_half_height;
    let viewport_max_y = camera_pos.y + world_viewport_half_height;

    // Convert world bounds to chunk coordinates
    let chunk_size = 32.0 * 13.0; // 416 world units per chunk

    // DIAGNOSTIC LOGGING
    log::info!("=== VIEWPORT CALCULATION DEBUG ===");
    log::info!("Camera pos: ({:.1}, {:.1})", camera_pos.x, camera_pos.y);
    log::info!("Current zoom: {:.3}", current_zoom);
    log::info!(
        "Center offset: ({:.1}, {:.1})",
        world_data.center_offset.x,
        world_data.center_offset.y
    );
    log::info!(
        "Viewport half size: {:.1} x {:.1}",
        world_viewport_half_width,
        world_viewport_half_height
    );
    log::info!(
        "Viewport bounds: x[{:.1}, {:.1}] y[{:.1}, {:.1}]",
        viewport_min_x,
        viewport_max_x,
        viewport_min_y,
        viewport_max_y
    );

    // Add padding for smooth loading
    let chunk_padding = 3;
    log::info!("Chunk size: {:.1}, padding: {}", chunk_size, chunk_padding);

    // Convert viewport bounds directly to chunk coordinates
    // Camera is already positioned relative to center_offset, so no additional offset needed
    let min_chunk_x_raw = (viewport_min_x / chunk_size).floor() as i32;
    let max_chunk_x_raw = (viewport_max_x / chunk_size).ceil() as i32;
    let min_chunk_z_raw = (viewport_min_y / chunk_size).floor() as i32;
    let max_chunk_z_raw = (viewport_max_y / chunk_size).ceil() as i32;

    // Apply padding
    let min_chunk_x = min_chunk_x_raw - chunk_padding;
    let max_chunk_x = max_chunk_x_raw + chunk_padding;
    let min_chunk_z = min_chunk_z_raw - chunk_padding;
    let max_chunk_z = max_chunk_z_raw + chunk_padding;

    log::debug!(
        "Viewport -> chunks: x[{}, {}] z[{}, {}]",
        min_chunk_x,
        max_chunk_x,
        min_chunk_z,
        max_chunk_z
    );

    // Convert viewport chunk coordinates to world chunk coordinates
    // Camera viewport chunks are relative to camera center, need to map to world coordinates [0, 239]
    let center_chunk_x = (world_data.center_offset.x / chunk_size) as i32;
    let center_chunk_z = (world_data.center_offset.y / chunk_size) as i32;

    let world_min_x = (min_chunk_x + center_chunk_x).max(0);
    let world_max_x = (max_chunk_x + center_chunk_x).min(239);
    let world_min_z = (min_chunk_z + center_chunk_z).max(0);
    let world_max_z = (max_chunk_z + center_chunk_z).min(239);

    // Collect chunks in viewport
    let mut chunks_in_viewport = HashSet::new();
    for chunk_x in world_min_x..=world_max_x {
        for chunk_z in world_min_z..=world_max_z {
            if world_data.chunks.contains_key(&(chunk_x, chunk_z)) {
                chunks_in_viewport.insert((chunk_x, chunk_z));
            }
        }
    }

    log::debug!(
        "Viewport culling: {} chunks in view",
        chunks_in_viewport.len()
    );

    // Despawn chunks that are no longer in viewport
    let mut despawned_count = 0;
    for (entity, dynamic_chunk) in chunk_query.iter() {
        if !chunks_in_viewport.contains(&dynamic_chunk.chunk_coords) {
            commands.entity(entity).despawn();
            spawned_chunks.chunks.remove(&dynamic_chunk.chunk_coords);
            despawned_count += 1;
        }
    }

    // Spawn new chunks that came into viewport
    let layout = HexLayout::pointy().with_hex_size(13.0);
    let mut spawned_count = 0;

    for &chunk_coords in &chunks_in_viewport {
        if !spawned_chunks.chunks.contains(&chunk_coords) {
            // Get chunk data
            let Some(chunk) = world_data.chunks.get(&chunk_coords) else {
                continue;
            };

            // Calculate chunk bounds for culling
            let chunk_center_x = chunk.chunk_x as f32 * chunk_size;
            let chunk_center_z = chunk.chunk_z as f32 * chunk_size;

            let chunk_bounds = ChunkBounds {
                chunk_coords,
                world_bounds: Rect::new(
                    chunk_center_x - world_data.center_offset.x - chunk_size / 2.0,
                    chunk_center_z - world_data.center_offset.y - chunk_size / 2.0,
                    chunk_size,
                    chunk_size,
                ),
            };

            // Generate combined mesh for chunk
            let (chunk_mesh, chunk_component) =
                create_chunk_mesh(chunk, &layout, world_data.center_offset);
            let mesh_handle = meshes.add(chunk_mesh);

            // Use white material to allow vertex colors to show through
            let material = materials.add(ColorMaterial::from(Color::WHITE));

            commands.spawn((
                Mesh2d(mesh_handle),
                MeshMaterial2d(material),
                Transform::IDENTITY,
                chunk_component,
                chunk_bounds,
                VisibleChunk,
                DynamicChunk { chunk_coords },
            ));

            spawned_chunks.chunks.insert(chunk_coords);
            spawned_count += 1;
        }
    }

    if spawned_count > 0 || despawned_count > 0 {
        log::debug!(
            "Viewport update: spawned {} chunks, despawned {} chunks, total visible: {}",
            spawned_count,
            despawned_count,
            chunks_in_viewport.len()
        );
    }
}
