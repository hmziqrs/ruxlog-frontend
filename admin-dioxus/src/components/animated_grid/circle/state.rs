#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn delta(self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }

    pub fn perpendicular(self) -> [Direction; 2] {
        match self {
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpawnEdge {
    Left,
    Right,
    Top,
    Bottom,
}

impl SpawnEdge {
    pub fn random() -> Self {
        use super::circles::random_usize;
        match random_usize(4) {
            0 => SpawnEdge::Left,
            1 => SpawnEdge::Right,
            2 => SpawnEdge::Top,
            _ => SpawnEdge::Bottom,
        }
    }
}

impl From<SpawnEdge> for Direction {
    fn from(edge: SpawnEdge) -> Self {
        match edge {
            SpawnEdge::Left => Direction::Right,
            SpawnEdge::Right => Direction::Left,
            SpawnEdge::Top => Direction::Down,
            SpawnEdge::Bottom => Direction::Up,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GridCircle {
    pub id: u64,
    pub col: i32,
    pub row: i32,
    pub travel_dir: Direction,
    pub moving: bool,
    pub respawning: bool,
    pub spawn_edge: SpawnEdge,
    pub alive: bool,
    pub just_side_stepped: bool,
    pub scale: f64,
    pub opacity: f64,
}
