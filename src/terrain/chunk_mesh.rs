use bevy::{
    math::Rect,
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};
use hexx::*;

use crate::terrain::{chunk::TerrainChunkState, color_utils::calculate_hex_color};

/// Component for chunk-level mesh entities
#[derive(Component)]
pub struct ChunkMesh {
    pub chunk_coords: (i32, i32),
    pub hex_count: usize,
    pub bounds: Rect,
}

/// Builder for combining multiple hex meshes into a single chunk mesh
pub struct ChunkMeshBuilder {
    vertices: Vec<Vec3>,
    colors: Vec<[f32; 4]>,
    normals: Vec<Vec3>,
    uvs: Vec<Vec2>,
    indices: Vec<u32>,
    current_vertex_offset: u32,
}

impl ChunkMeshBuilder {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            colors: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
            current_vertex_offset: 0,
        }
    }

    /// Add a hex mesh to the combined mesh at the specified world position
    pub fn add_hex(&mut self, layout: &HexLayout, world_pos: Vec2, color: Color) {
        // Generate hex mesh data
        let mesh_info = PlaneMeshBuilder::new(layout)
            .facing(Vec3::Z)
            .with_scale(Vec3::splat(0.98))
            .center_aligned()
            .build();

        let vertex_count = mesh_info.vertices.len() as u32;

        // Transform vertices to world position and add to combined mesh
        for vertex in mesh_info.vertices {
            let world_vertex =
                Vec3::new(vertex[0] + world_pos.x, vertex[1] + world_pos.y, vertex[2]);
            self.vertices.push(world_vertex);
        }

        // Add vertex colors (same color for all vertices of this hex)
        let linear_color = color.to_linear();
        let color_array = [
            linear_color.red,
            linear_color.green,
            linear_color.blue,
            linear_color.alpha,
        ];
        for _ in 0..vertex_count {
            self.colors.push(color_array);
        }

        // Add normals
        for normal in mesh_info.normals {
            self.normals
                .push(Vec3::new(normal[0], normal[1], normal[2]));
        }

        // Add UVs
        for uv in mesh_info.uvs {
            self.uvs.push(Vec2::new(uv[0], uv[1]));
        }

        // Add indices with vertex offset
        for index in mesh_info.indices {
            self.indices.push(index as u32 + self.current_vertex_offset);
        }

        self.current_vertex_offset += vertex_count;
    }

    /// Build the final combined mesh
    pub fn build(self) -> Mesh {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );

        // Convert Vec3 to [f32; 3] for mesh attributes
        let vertices: Vec<[f32; 3]> = self.vertices.iter().map(|v| v.to_array()).collect();
        let normals: Vec<[f32; 3]> = self.normals.iter().map(|n| n.to_array()).collect();
        let uvs: Vec<[f32; 2]> = self.uvs.iter().map(|uv| uv.to_array()).collect();

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, self.colors);
        mesh.insert_indices(Indices::U32(self.indices));

        mesh
    }

    /// Get the number of hexes added to this mesh
    pub fn hex_count(&self) -> usize {
        (self.current_vertex_offset / 6) as usize // Assuming 6 vertices per hex
    }
}

/// Create a batched mesh for an entire chunk
pub fn create_chunk_mesh(
    chunk: &TerrainChunkState,
    layout: &HexLayout,
    center_offset: Vec2,
) -> (Mesh, ChunkMesh) {
    let mut builder = ChunkMeshBuilder::new();
    let cells = chunk.cells();

    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    // Add each hex in the chunk to the combined mesh
    for cell in &cells {
        let hex = layout.world_pos_to_hex(Vec2::new(cell.cell_x as f32, cell.cell_z as f32));
        let pos = layout.hex_to_world_pos(hex);

        // Apply center offset to center the map around (0,0)
        let world_pos = pos - center_offset;

        // Track bounds for frustum culling later
        min_x = min_x.min(world_pos.x);
        max_x = max_x.max(world_pos.x);
        min_y = min_y.min(world_pos.y);
        max_y = max_y.max(world_pos.y);

        // Calculate biome-based color
        let color = calculate_hex_color(cell.biome, cell.elevation);

        builder.add_hex(layout, world_pos, color);
    }

    let chunk_bounds = Rect::new(min_x, min_y, max_x - min_x, max_y - min_y);
    let hex_count = cells.len();

    let mesh = builder.build();
    let chunk_component = ChunkMesh {
        chunk_coords: (chunk.chunk_x, chunk.chunk_z),
        hex_count,
        bounds: chunk_bounds,
    };

    (mesh, chunk_component)
}
