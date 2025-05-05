use bevy::prelude::*;
use chilly::grid::Grid;
use chilly::import::{Import, Level};
use chilly::solver::{Strategy, PlayerStart, solve};
use chilly::helpers::print_ways_html;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: None,
            ..default()
        }))

        .insert_resource(Grid::new())

        // NoDuplicateNodes | NoDuplicateEdges
        .insert_resource(Strategy::NoDuplicateEdges)

        // (7, 1) | (11, 10)
        .insert_resource(PlayerStart((7, 1)))

        // MiniEiersuche, KeinHaltZweimal, KeineDoppeltenWege
        .add_plugins(Import::new(Level::MiniEiersuche))

        .add_systems(Startup, print_ways_html)
        .add_systems(Last, solve)
        .run();
}
