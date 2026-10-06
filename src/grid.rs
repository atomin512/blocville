#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileType {
    Empty,
    Residential,
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub types: Vec<TileType>,
    pub pollutions: Vec<u8>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let len_of_vector = width * height;
        Self {
            width,
            height,
            types: vec![TileType::Empty; len_of_vector],
            pollutions: vec![0u8; len_of_vector],
        }
    }

    pub fn get_tile_pollution(&self, x: usize, y: usize) -> Option<&u8> {
        if x < self.width && y < self.height {
            self.pollutions.get(x + y * self.width)
        } else {
            None
        }
    }

    pub fn get_tile_pollution_mut(&mut self, x: usize, y: usize) -> Option<&mut u8> {
        if x < self.width && y < self.height {
            self.pollutions.get_mut(x + y * self.width)
        } else {
            None
        }
    }

    pub fn get_tile_type(&self, x: usize, y: usize) -> Option<&TileType> {
        if x < self.width && y < self.height {
            self.types.get(x + y * self.width)
        } else {
            None
        }
    }

    pub fn get_tile_type_mut(&mut self, x: usize, y: usize) -> Option<&mut TileType> {
        if x < self.width && y < self.height {
            self.types.get_mut(x + y * self.width)
        } else {
            None
        }
    }
}
