use crate::config::*;

pub struct GridDelta {
    pub width: usize,
    pub height: usize,
    pub pollutions: Vec<i8>,
}

impl GridDelta {
    pub fn new(width: usize, height: usize) -> Self {
        let len_of_vector = width * height;
        Self {
            width,
            height,
            pollutions: vec![0i8; len_of_vector],
        }
    }

    pub fn clear(&mut self) {
        self.pollutions.fill(0);
    }

    pub fn apply_8_neighbors_pollution(&mut self, x: usize, y: usize, delta_pollution: i8) {
        for (dx, dy) in OFFSETS_OF_8 {
            let Some(nx) = x.checked_add_signed(dx) else { continue };
            let Some(ny) = y.checked_add_signed(dy) else { continue };
            if nx < self.width && ny < self.height {
                let idx = ny * self.width + nx;
                self.pollutions[idx] = self.pollutions[idx].saturating_add(delta_pollution);
            }
        }
    }
}