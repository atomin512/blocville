use crate::config;
use crate::grid;

fn apply_tile_pollution(world: &grid::Grid, buffer: &mut grid::Grid) {
    for (kind, pollution) in world.types.iter().zip(buffer.pollutions.iter_mut()) {
        match kind {
            grid::TileType::Residential => {
                *pollution = pollution.saturating_add_signed(config::RESIDENTIAL_POLLUTION_PER_TICK);
            }
            grid::TileType::Empty => {
                *pollution = pollution.saturating_add_signed(config::EMPTY_POLLUTION_PER_TICK);
            }
        }
    }
}

fn apply_pollution_spreading(world: &grid::Grid, world_buffer: &mut grid::Grid) {
    for (idx, pollution) in world.pollutions.iter().enumerate() {
        if *pollution < config::POLLUTION_SPREAD_THRESHOLD {
            continue;
        }
        world_buffer.apply_8_neighbors_pollution(
            idx % world.width(),
            (idx - idx % world.width()) / world.width(),
            (pollution / config::POLLUTION_SPREAD_THRESHOLD) as i8,
        );
        world_buffer.pollutions[idx] = world_buffer.pollutions[idx].saturating_add_signed(-((pollution / 2) as i8));
    }
}

pub fn tick(world: &mut grid::Grid, world_buffer: &mut grid::Grid) {
    *world_buffer = world.clone();
    apply_tile_pollution(world, world_buffer);
    apply_pollution_spreading(world, world_buffer);
    *world = world_buffer.clone();
}