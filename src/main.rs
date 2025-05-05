use bevy::prelude::*;
use chilly::grid::Grid;
use chilly::import::Import;
use chilly::solver::{Strategy, PlayerStart, solve};
use chilly::helpers::print_ways_html;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: None,
            ..default()
        }))
        .insert_resource(Grid::new())
        .insert_resource(Strategy::NoDuplicateEdges)
        // .insert_resource(Strategy::NoDuplicateEdges)
        .insert_resource(PlayerStart((7, 1))) // 7, 1 // 11, 10
        // .add_plugins(Import::new("assets/keine-doppelten-wege.txt".to_string()))
        // .add_plugins(Import::new("assets/kein-halt-zweimal.txt".to_string()))
        .add_plugins(Import::new("assets/mini-eiersuche.txt".to_string()))
        .add_systems(Startup, print_ways_html)
        .add_systems(Last, solve)
        .run();
}
