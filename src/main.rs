mod config;
mod grid;
mod simulation;

fn main() {
    // Creating an empty grid with no pollution
    let mut grid = grid::Grid::new(2048, 2048);
    let mut buffer = grid::GridDelta::new(2048, 2048);

    // Spawning Residential tile at 0:0 coordinates
    if let Some(t) = grid.get_tile_type_mut(0, 0) {
        *t = grid::TileType::Residential;
    }

    for _ in 0..200 {
        simulation::tick(&mut grid, &mut buffer);
    }

    println!(
        "{}, {}, {}",
        grid.pollutions[0], grid.pollutions[1], grid.pollutions[2]
    );
}