use crate::terrain::biome_colors_range::{BIOME_COLORS, ColorRange};
use bevy::prelude::Color;

/// Water color for hexes below elevation 0
pub const WATER_COLOR: Color = Color::srgb_u8(0, 105, 148);

/// Calculates the color for a hex based on biome and elevation
///
/// # Arguments
/// * `biome` - The biome type as u32
/// * `elevation` - The elevation value as i16
///
/// # Returns
/// The calculated Color based on biome and elevation
pub fn calculate_hex_color(biome: u32, elevation: i16) -> Color {
    // Handle water (below elevation 0)
    if elevation < 0 {
        return WATER_COLOR;
    }

    // Get the color range for this biome
    let color_range = match BIOME_COLORS.get(&biome) {
        Some(range) => *range,
        None => {
            // Fallback to a default color if biome not found
            ColorRange {
                start: Color::srgb_u8(128, 128, 128),
                end: Color::srgb_u8(200, 200, 200),
            }
        }
    };

    // Normalize elevation to a 0-1 range for interpolation
    // Using actual elevation range (0 to 50) based on game data
    let normalized_elevation = (elevation as f32 / 50.0).clamp(0.0, 1.0);

    // Interpolate between start and end colors based on elevation
    let start = color_range.start;
    let end = color_range.end;

    // Convert colors to linear RGB for interpolation
    let start_linear = start.to_linear();
    let end_linear = end.to_linear();

    Color::srgb(
        start_linear.red + (end_linear.red - start_linear.red) * normalized_elevation,
        start_linear.green + (end_linear.green - start_linear.green) * normalized_elevation,
        start_linear.blue + (end_linear.blue - start_linear.blue) * normalized_elevation,
    )
}

/// Pre-calculates colors for common biome/elevation combinations
/// This can be used for optimization in the rendering system
pub fn create_color_cache() -> Vec<Color> {
    // Create a cache for common biome/elevation combinations
    // This is a placeholder for future optimization
    Vec::new()
}
