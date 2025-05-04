use bevy::prelude::*;
use chilly::grid::Grid;
use chilly::import::Import;
use chilly::solver::{Strategy, PlayerStart, solve, print_grid, print_ways};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: None,
            ..default()
        }))
        .insert_resource(Grid::new())
        .insert_resource(Strategy::NoDuplicateEdges)
        .insert_resource(PlayerStart((7, 1)))
        .add_plugins(Import::new("assets/mini-eiersuche.txt".to_string()))
        .add_systems(Startup, (print_grid, print_ways).chain())
        .add_systems(Last, solve)
        .run();
}
