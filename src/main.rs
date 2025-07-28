use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

pub mod terrain;

use terrain::{
    chunk::TerrainChunkState,
    dynamic_chunks::{SpawnedChunks, update_dynamic_chunks},
    world_data::WorldData,
};

pub fn main() {
    // og4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (1_000.0, 1_000.0).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(bevy::log::LogPlugin { ..default() }),
        )
        .init_resource::<WorldData>()
        .init_resource::<SpawnedChunks>()
        .add_systems(Startup, (load_world_data, setup_camera).chain())
        .add_systems(Update, (camera_controls, update_dynamic_chunks))
        .run();

    log::info!("Done.");
}

/// 2D camera setup - position camera at world center
fn setup_camera(mut commands: Commands, world_data: Res<WorldData>) {
    // Position camera at world center so we can see terrain immediately
    let camera_pos = Vec3::new(
        -world_data.center_offset.x,
        -world_data.center_offset.y,
        0.0,
    );

    commands.spawn((Camera2d, Transform::from_translation(camera_pos)));

    log::info!(
        "Camera positioned at ({:.1}, {:.1}) to center on world",
        camera_pos.x,
        camera_pos.y
    );
}

/// Load all world data into memory for dynamic chunk loading
fn load_world_data(mut world_data: ResMut<WorldData>) {
    let regions = TerrainChunkState::from_dir("./data").expect("Could not read data dir!");

    for region in regions {
        log::info!("Loaded {} chunks from region file", region.len());

        // Filter out invalid chunks
        let region_len = region.len();
        let valid_chunks: Vec<_> = region
            .into_iter()
            .enumerate()
            .filter_map(|(idx, chunk)| {
                if chunk.chunk_x == 0 && chunk.chunk_z == 0 {
                    if idx == 0 {
                        log::info!("Including valid origin chunk at index {idx}");
                        Some(chunk)
                    } else {
                        None // Filter out invalid instanced chunks at (0,0)
                    }
                } else {
                    Some(chunk)
                }
            })
            .collect();

        log::info!(
            "After filtering: {} valid chunks (removed {} invalid chunks)",
            valid_chunks.len(),
            region_len - valid_chunks.len()
        );

        // Add region to world data
        world_data.add_region(valid_chunks);
    }

    // Calculate center offset after all regions are loaded
    world_data.finalize();

    // Log coordinate ranges for debugging
    let chunk_coords: Vec<_> = world_data.chunks.keys().collect();
    let min_x = chunk_coords.iter().map(|(x, _)| *x).min().unwrap_or(0);
    let max_x = chunk_coords.iter().map(|(x, _)| *x).max().unwrap_or(0);
    let min_z = chunk_coords.iter().map(|(_, z)| *z).min().unwrap_or(0);
    let max_z = chunk_coords.iter().map(|(_, z)| *z).max().unwrap_or(0);

    log::info!(
        "World data loaded: {} total chunks, center offset: ({:.1}, {:.1})",
        world_data.chunks.len(),
        world_data.center_offset.x,
        world_data.center_offset.y
    );
    log::info!(
        "Chunk coordinate ranges: X=[{}, {}], Z=[{}, {}]",
        min_x,
        max_x,
        min_z,
        max_z
    );
}

/// Camera controls for zooming and panning
fn camera_controls(
    mut scroll_events: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut Camera, &mut Projection, &mut Transform)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut motion_events: EventReader<MouseMotion>,
) {
    let zoom_speed = 0.1;
    let pan_speed = 1.0;

    for scroll in scroll_events.read() {
        for (_camera, mut projection, _transform) in camera_query.iter_mut() {
            if let Projection::Orthographic(ref mut ortho) = *projection {
                let zoom_factor = 1.0 - scroll.y * zoom_speed;
                ortho.scale = (ortho.scale * zoom_factor).clamp(0.1, 10.0);
            }
        }
    }

    if mouse_input.pressed(MouseButton::Left) {
        let mut total_motion = Vec2::ZERO;
        for motion in motion_events.read() {
            total_motion += motion.delta;
        }

        if total_motion != Vec2::ZERO {
            for (_camera, projection, mut transform) in camera_query.iter_mut() {
                if let Projection::Orthographic(ref ortho) = *projection {
                    // Invert motion for natural panning
                    let pan_delta = -total_motion * pan_speed * ortho.scale;
                    transform.translation += Vec3::new(pan_delta.x, -pan_delta.y, 0.0);
                }
            }
        }
    }
}
