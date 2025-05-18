
use bevy::prelude::*;
// use bevy::sprite::SpriteBundle;
use crate::grid::{CellType, Grid};
pub struct RenderGrid;

const CELLSIZE : f32 = 32.0;

impl Plugin for RenderGrid {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Last, change_resolution);
    }
}

fn setup(
    grid: Res<Grid>,
    mut commands: Commands,
) {
    commands.spawn(Camera2d);

    let (size_x, size_y) = grid.size;
    let offset_x = size_x as f32 * CELLSIZE / 2.0 - CELLSIZE / 2.0;
    let offset_y = size_y as f32 * CELLSIZE / 2.0 - CELLSIZE / 2.0;

    for cell in &grid.cells {
        let sprite_color = match cell.cell_type {
            CellType::Empty =>     Color::hsl(200.0, 0.8, 0.7), // lightblue
            CellType::Obstacle =>  Color::hsl(0.0, 0.0, 0.3),  // gray
            CellType::Portal(_) => Color::hsl(200.0, 0.8, 0.3), // blue
            CellType::Exit =>      Color::hsl(0.0, 1.0, 0.4) // red
        };

        let cx = (cell.id % size_x) as f32; // the cells row and column
        let cy = (cell.id / size_x) as f32;

        let x = cx * CELLSIZE - offset_x; // the cells screen coordinates
        let y = cy * CELLSIZE - offset_y;

        commands.spawn((
            Sprite {
                color: sprite_color,
                custom_size: Some(Vec2::splat(CELLSIZE)),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, 0.0)),
        ));
    }
}

fn change_resolution(
    mut window: Single<&mut Window>,
    grid: Res<Grid>,
) {
    let (x, y) = grid.size;
    let res = Vec2::new(
        (x as f32 * CELLSIZE).round(),
        (y as f32 * CELLSIZE).round(),
    );
    window.resolution.set(res.x, res.y);
}

