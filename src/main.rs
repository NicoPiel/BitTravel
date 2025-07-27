use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    platform::collections::HashMap,
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};
use hexx::*;

pub mod terrain;

use terrain::{chunk::TerrainChunkState, color_utils::calculate_hex_color};

/// World size of the hexagons (outer radius)
const HEX_SIZE: f32 = 13.0;

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
        .add_systems(Startup, (setup_camera, setup_grid))
        .add_systems(Update, camera_controls)
        .run();

    log::info!("Done.");
}

/// 2D camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Hex grid setup
fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let layout = HexLayout::pointy().with_hex_size(HEX_SIZE);
    // mesh
    let mesh = hexagonal_plane(&layout);
    let mesh_handle = meshes.add(mesh);

    let regions = TerrainChunkState::from_dir("./data").expect("Could not read data dir!");

    for region in regions {
        log::info!("Loaded {} chunks from region file", region.len());

        // More sophisticated filtering: allow first (0,0) chunk, filter out the rest
        let all_chunks: Vec<_> = region
            .iter()
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
            "After filtering: {} valid chunks (removed {} invalid (0,0) chunks)",
            all_chunks.len(),
            region.len() - all_chunks.len()
        );

        // Calculate the center of all cells to offset coordinates around (0,0)
        let all_cells: Vec<_> = all_chunks.iter().flat_map(|chunk| chunk.cells()).collect();

        let (min_x, max_x, min_z, max_z) = all_cells.iter().fold(
            (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
            |(min_x, max_x, min_z, max_z), cell| {
                (
                    min_x.min(cell.cell_x),
                    max_x.max(cell.cell_x),
                    min_z.min(cell.cell_z),
                    max_z.max(cell.cell_z),
                )
            },
        );

        let center_x = (min_x + max_x) as f32 / 2.0;
        let center_z = (min_z + max_z) as f32 / 2.0;

        log::info!(
            "Hex field bounds: x[{min_x} to {max_x}], z[{min_z} to {max_z}], centering at ({center_x:.1}, {center_z:.1})"
        );

        let entities: HashMap<Hex, Entity> = all_cells
            .into_iter()
            .map(|cell| {
                let hex = layout.world_pos_to_hex(vec2(cell.cell_x as f32, cell.cell_z as f32));
                let pos = layout.hex_to_world_pos(hex);

                // Calculate biome-based color
                let biome = cell.biome;
                let elevation = cell.elevation;
                let color = calculate_hex_color(biome, elevation);
                // log::info!("{color:?}");
                let material = materials.add(ColorMaterial::from(color));

                let id = commands
                    .spawn((
                        Mesh2d(mesh_handle.clone()),
                        MeshMaterial2d(material),
                        Transform::from_xyz(pos.x, pos.y, 0.0),
                    ))
                    .id();
                (hex, id)
            })
            .collect();

        // DEBUG: Log total number of entities created
        log::info!("Created {} hex entities for rendering", entities.len());
    }
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

/// Compute a bevy mesh from the layout
fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        .with_scale(Vec3::splat(0.98))
        .center_aligned()
        .build();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}
