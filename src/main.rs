use bevy::{
    color::palettes::css::{AQUA, RED, WHITE},
    platform::collections::HashMap,
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
    window::PrimaryWindow,
};
use hexx::*;

pub mod terrain;

use terrain::chunk::TerrainChunkState;

/// World size of the hexagons (outer radius)
const HEX_SIZE: f32 = 13.0;

pub fn main() {
    // og4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    App::new()
        .init_resource::<HighlightedHexes>()
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
        .add_systems(Update, (handle_input).chain())
        .run();

    log::info!("Done.");
}

#[derive(Debug, Default, Resource)]
struct HighlightedHexes {
    pub selected: Hex,
    pub halfway: Hex,
}

#[derive(Debug, Resource)]
struct Map {
    layout: HexLayout,
    entities: HashMap<Hex, Entity>,
    selected_material: Handle<ColorMaterial>,
    default_material: Handle<ColorMaterial>,
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
    // materials
    let selected_material = materials.add(Color::Srgba(RED));
    let default_material = materials.add(Color::Srgba(WHITE));
    // mesh
    let mesh = hexagonal_plane(&layout);
    let mesh_handle = meshes.add(mesh);

    let region = TerrainChunkState::from_bsatn("./data/5.bsatn");

    let entities = region
        .iter()
        .flat_map(|chunks| chunks.iter())
        .flat_map(|chunk| chunk.cells())
        .map(|cell| {
            let hex = layout.world_pos_to_hex(vec2(cell.cell_x as f32, cell.cell_z as f32));

            let pos = layout.hex_to_world_pos(hex);

            let id = commands
                .spawn((
                    Mesh2d(mesh_handle.clone()),
                    MeshMaterial2d(default_material.clone_weak()),
                    Transform::from_xyz(pos.x, pos.y, 0.0),
                ))
                .id();
            (hex, id)
        })
        .collect();
    commands.insert_resource(Map {
        layout,
        entities,
        selected_material,
        default_material,
    });
}

/// Input interaction
fn handle_input(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    map: Res<Map>,
    mut highlighted_hexes: ResMut<HighlightedHexes>,
) -> Result {
    let window = windows.single()?;
    let (camera, cam_transform) = cameras.single()?;
    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p).ok())
    {
        let coord = map.layout.world_pos_to_hex(pos);
        if let Some(entity) = map.entities.get(&coord).copied() {
            if coord == highlighted_hexes.selected {
                return Ok(());
            }
            commands
                .entity(map.entities[&highlighted_hexes.selected])
                .insert(MeshMaterial2d(map.default_material.clone_weak()));
            commands
                .entity(map.entities[&highlighted_hexes.halfway])
                .insert(MeshMaterial2d(map.default_material.clone_weak()));
            // Make the half selction red
            highlighted_hexes.halfway = coord / 2;
            commands
                .entity(map.entities[&highlighted_hexes.halfway])
                .insert(MeshMaterial2d(map.selected_material.clone_weak()));
            // Make the selected tile red
            commands
                .entity(entity)
                .insert(MeshMaterial2d(map.selected_material.clone_weak()));
            highlighted_hexes.selected = coord;
        }
    }
    Ok(())
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

fn test() {
    let regions = match TerrainChunkState::from_dir("./data") {
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

        /*let ending_x = &region
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

        let unique_biomes = &region
            .iter()
            .flat_map(|r| r.biomes.iter())
            .unique()
            .collect_vec();*/

        log::info!("Region: {region_index}");
        log::info!("Chunks: {chunks}, cells: {cells}");
        // log::info!("Ending chunk coords: [{ending_x}:{ending_y}]");
        // log::info!("Caves? {caves}");
        // log::info!("Chunks without instances {:?}", chunks - caves);
        // log::info!("Unique densities {:?}", unique_densities);
        // log::info!("Unique biomes: {:?}", unique_biomes);
        log::info!("----------------------------");

        region_index += 1;
    }

    let total_chunks = &regions.iter().fold(0, |acc, r| acc + r.len());

    log::info!("Chunks in total: {total_chunks}",);
}
