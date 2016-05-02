use specs;

/// Position

pub struct Position {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Position {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Position {
            x: x,
            y: y,
            z: z,
        }
    }
}

impl specs::Component for Position {
    type Storage = specs::VecStorage<Position>;
}

/// PlayerControlled

pub struct PlayerControlled;

impl Default for PlayerControlled {
    fn default() -> Self {
        PlayerControlled
    }
}

impl specs::Component for PlayerControlled {
    type Storage = specs::VecStorage<PlayerControlled>;
}

pub struct Visible {
    pub mark: u8,
}

impl Default for Visible {
    fn default() -> Self {
        Visible {
            mark: 1,
        }
    }
}

impl specs::Component for Visible {
    type Storage = specs::VecStorage<Visible>;
}
