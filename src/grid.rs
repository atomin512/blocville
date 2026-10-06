use crate::config;
use glam::USizeVec2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileType {
    Empty,
    Residential,
}

#[inline]
fn checked_offset(pos: USizeVec2, dx: isize, dy: isize) -> Option<USizeVec2> {
    Some(USizeVec2::new(
        pos.x.checked_add_signed(dx)?,
        pos.y.checked_add_signed(dy)?,
    ))
}

#[derive(Clone)]
pub struct Grid {
    size: USizeVec2,
    pub types: Vec<TileType>,
    pub pollutions: Vec<u8>,
}

impl Grid {
    pub fn new(size: USizeVec2) -> Self {
        let len = size.x * size.y;
        Self {
            size,
            types: vec![TileType::Empty; len],
            pollutions: vec![0u8; len],
        }
    }

    #[inline]
    pub fn size(&self) -> USizeVec2 {
        self.size
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.size.x
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.size.y
    }

    #[inline]
    pub fn index(&self, pos: USizeVec2) -> Option<usize> {
        (pos.x < self.size.x && pos.y < self.size.y)
            .then(|| pos.y * self.size.x + pos.x)
    }

    pub fn get_tile_pollution(&self, pos: USizeVec2) -> Option<&u8> {
        self.index(pos).map(|i| &self.pollutions[i])
    }

    pub fn get_tile_pollution_mut(&mut self, pos: USizeVec2) -> Option<&mut u8> {
        let i = self.index(pos)?;
        self.pollutions.get_mut(i)
    }

    pub fn get_tile_type(&self, pos: USizeVec2) -> Option<&TileType> {
        self.index(pos).map(|i| &self.types[i])
    }

    pub fn get_tile_type_mut(&mut self, pos: USizeVec2) -> Option<&mut TileType> {
        let i = self.index(pos)?;
        self.types.get_mut(i)
    }

    pub fn apply_8_neighbors_pollution(&mut self, pos: USizeVec2, delta_pollution: i8) {
        for (dx, dy) in config::OFFSETS_OF_8 {
            let Some(neighbor) = checked_offset(pos, dx, dy) else {
                continue;
            };
            let Some(idx) = self.index(neighbor) else {
                continue;
            };
            self.pollutions[idx] =
                self.pollutions[idx].saturating_add_signed(delta_pollution);
        }
    }
}