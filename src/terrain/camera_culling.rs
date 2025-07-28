use bevy::math::Rect;
use bevy::prelude::*;

/// Component to track chunks that should be rendered based on camera view
#[derive(Component)]
pub struct VisibleChunk;

/// Component storing chunk position and bounds for culling calculations
#[derive(Component)]
pub struct ChunkBounds {
    pub chunk_coords: (i32, i32),
    pub world_bounds: Rect,
}

/// Calculate which chunks are visible from the camera
pub fn calculate_visible_chunks(
    camera_transform: &Transform,
    camera_projection: &OrthographicProjection,
    _chunk_size: f32, // World size of a chunk
) -> Rect {
    // Calculate camera view bounds in world coordinates
    let camera_pos = camera_transform.translation.truncate();
    let scale = camera_projection.scale;

    // Assume window size for orthographic projection bounds
    // In a real implementation, you'd get this from the window
    let half_width = 500.0 * scale; // Half window width scaled
    let half_height = 500.0 * scale; // Half window height scaled

    Rect::new(
        camera_pos.x - half_width,
        camera_pos.y - half_height,
        half_width * 2.0,
        half_height * 2.0,
    )
}

/// Check if a chunk intersects with the camera view
pub fn chunk_in_view(chunk_bounds: &Rect, view_bounds: &Rect) -> bool {
    // Add some padding to avoid pop-in at edges
    let padding = 50.0;
    let padded_view = Rect::new(
        view_bounds.min.x - padding,
        view_bounds.min.y - padding,
        view_bounds.width() + padding * 2.0,
        view_bounds.height() + padding * 2.0,
    );

    // Check for intersection
    !(chunk_bounds.max.x < padded_view.min.x
        || chunk_bounds.min.x > padded_view.max.x
        || chunk_bounds.max.y < padded_view.min.y
        || chunk_bounds.min.y > padded_view.max.y)
}

/// System that updates chunk visibility based on camera position
pub fn update_chunk_visibility(
    camera_query: Query<(&Transform, &Projection), With<Camera>>,
    mut chunk_query: Query<(Entity, &ChunkBounds, Option<&VisibleChunk>)>,
    mut commands: Commands,
) {
    // Get camera info
    let Ok((camera_transform, projection)) = camera_query.single() else {
        return;
    };

    let Projection::Orthographic(camera_projection) = projection else {
        return; // Only handle orthographic cameras
    };

    // Calculate current view bounds
    let view_bounds = calculate_visible_chunks(camera_transform, camera_projection, 32.0 * 13.0);

    let mut visible_count = 0;
    let mut hidden_count = 0;

    // Update visibility for each chunk
    for (entity, chunk_bounds, currently_visible) in chunk_query.iter_mut() {
        let should_be_visible = chunk_in_view(&chunk_bounds.world_bounds, &view_bounds);

        match (currently_visible.is_some(), should_be_visible) {
            (false, true) => {
                // Chunk should become visible
                commands.entity(entity).insert(VisibleChunk);
                visible_count += 1;
            }
            (true, false) => {
                // Chunk should become hidden
                commands.entity(entity).remove::<VisibleChunk>();
                hidden_count += 1;
            }
            (true, true) => visible_count += 1,
            (false, false) => hidden_count += 1,
        }
    }

    // Only log when visibility changes significantly
    if visible_count + hidden_count > 0 && (visible_count < 1000) {
        log::debug!("Chunks: {} visible, {} hidden", visible_count, hidden_count);
    }
}
