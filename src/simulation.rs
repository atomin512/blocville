use crate::config;
use crate::grid;

fn apply_tile_pollution(grid: &grid::Grid, buffer: &mut grid::GridDelta) {
    for (kind, pollution) in grid.types.iter().zip(buffer.pollutions.iter_mut()) {
        match kind {
            grid::TileType::Residential => {
                *pollution = pollution.saturating_add(config::RESIDENTIAL_POLLUTION_PER_TICK);
            }
            grid::TileType::Empty => {
                *pollution = pollution.saturating_add(config::EMPTY_POLLUTION_PER_TICK);
            }
        }
    }
}

fn apply_pollution_spreading(grid: &grid::Grid, buffer: &mut grid::GridDelta) {
    for (idx, pollution) in grid.pollutions.iter().enumerate() {
        if *pollution < config::POLLUTION_SPREAD_THRESHOLD {
            continue;
        }
        buffer.apply_8_neighbors_pollution(
            idx % grid.width,
            (idx - idx % grid.width) / grid.width,
            (pollution / config::POLLUTION_SPREAD_THRESHOLD) as i8,
        );
        buffer.pollutions[idx] = buffer.pollutions[idx].saturating_sub((pollution / 2) as i8);
    }
}

fn apply_buffer(grid: &mut grid::Grid, buffer: &mut grid::GridDelta) {
    for (idx, pollution) in grid.pollutions.iter_mut().enumerate() {
        *pollution = pollution.saturating_add_signed(buffer.pollutions[idx]);
        buffer.pollutions[idx] = 0;
    }
}

// Should apply_buffer be after every step or no?
pub fn tick(grid: &mut grid::Grid, buffer: &mut grid::GridDelta) {
    apply_tile_pollution(grid, buffer);
    //apply_buffer(grid, buffer); // ???
    apply_pollution_spreading(grid, buffer);
    apply_buffer(grid, buffer);
}