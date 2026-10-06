mod config;
mod grid;
mod simulation;

use std::time::{Duration, Instant};

fn main() {
    // Creating an empty grid with no pollution
    let mut world = grid::Grid::new(2048, 2048);
    let mut world_buffer = world.clone();

    // Spawning Residential tile at 0:0 coordinates
    if let Some(t) = world.get_tile_type_mut(0, 0) {
        *t = grid::TileType::Residential;
    }

    let start = Instant::now();
    for _ in 0..100 {
        simulation::tick(&mut world, &mut world_buffer);
    }
    let duration = start.elapsed();

    println!("simulation took {:?}", duration);

    println!(
        "{}, {}, {}, {}",
        world.pollutions[0], world.pollutions[1], world.pollutions[2], world.pollutions[3]
    );
}