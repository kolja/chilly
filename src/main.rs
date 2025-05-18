use bevy::prelude::*;
use chilly::grid::Grid;
use chilly::render_grid::RenderGrid;
use chilly::import::{Import, Level};
use chilly::solver::{Strategy, PlayerStart}; // solve

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
            // .set(WindowPlugin {
            //     primary_window: None,
            //     ..default()
            // }

        .insert_resource(Grid::new())

        // NoDuplicateNodes | NoDuplicateEdges
        .insert_resource(Strategy::NoDuplicateEdges)

        // (7, 1) | (11, 10)
        .insert_resource(PlayerStart((7, 1)))

        // MiniEiersuche, KeinHaltZweimal, KeineDoppeltenWege
        .add_plugins(Import::new(Level::KeinHaltZweimal))
        .add_plugins(RenderGrid)

        // .add_systems(Last, solve)
        .run();
}
