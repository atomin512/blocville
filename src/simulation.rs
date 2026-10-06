use crate::config;
use crate::grid;
use glam::USizeVec2;

fn apply_tile_pollution(world: &grid::Grid, buffer: &mut grid::Grid) {
    for (kind, pollution) in world.types.iter().zip(buffer.pollutions.iter_mut()) {
        let delta = match kind {
            grid::TileType::Residential => config::RESIDENTIAL_POLLUTION_PER_TICK,
            grid::TileType::Empty => config::EMPTY_POLLUTION_PER_TICK,
        };
        *pollution = pollution.saturating_add_signed(delta);
    }
}

fn apply_pollution_spreading(world: &grid::Grid, world_buffer: &mut grid::Grid) {
    let width = world.width();
    for (idx, pollution) in world.pollutions.iter().enumerate() {
        if *pollution < config::POLLUTION_SPREAD_THRESHOLD {
            continue;
        }
        let pos = USizeVec2::new(idx % width, idx / width);
        world_buffer.apply_8_neighbors_pollution(
            pos,
            (pollution / config::POLLUTION_SPREAD_THRESHOLD) as i8,
        );
        world_buffer.pollutions[idx] =
            world_buffer.pollutions[idx].saturating_add_signed(-((pollution / 2) as i8));
    }
}

pub fn tick(world: &mut grid::Grid, world_buffer: &mut grid::Grid) {
    *world_buffer = world.clone();
    apply_tile_pollution(world, world_buffer);
    apply_pollution_spreading(world, world_buffer);
    *world = world_buffer.clone();
}