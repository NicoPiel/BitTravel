#[derive(Clone, PartialEq, Debug, Copy, Eq, Hash)]
pub struct Cell {
    pub cell_x: i32,
    pub cell_z: i32,
    pub biome: u32,
    pub elevation: i16,
}
