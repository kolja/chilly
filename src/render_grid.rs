use bevy::prelude::*;
use crate::grid::{CellType, Direction, Grid};
pub struct RenderGrid;

const CELLSIZE: f32 = 64.0;

impl Plugin for RenderGrid {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Last, change_resolution);
    }
}

fn setup(grid: Res<Grid>, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let (size_x, size_y) = grid.size;
    let offset_x = size_x as f32 * CELLSIZE / 2.0 - CELLSIZE / 2.0;
    let offset_y = size_y as f32 * CELLSIZE / 2.0 - CELLSIZE / 2.0;

    let font = asset_server.load("FiraCode-Bold.ttf");

    let fira = TextFont {
        font: font.clone(),
        font_size: 12.0,
        ..default()
    };

    for cell in &grid.cells {
        let sprite_color = match cell.cell_type {
            CellType::Empty => Color::hsl(200.0, 0.8, 0.7), // lightblue
            CellType::Obstacle => Color::hsl(0.0, 0.0, 0.3), // gray
            CellType::Portal(_) => Color::hsl(200.0, 0.8, 0.3), // blue
            CellType::Exit => Color::hsl(0.0, 1.0, 0.4),    // red
        };

        let cx = (cell.id % size_x) as f32; // the cells row and column
        let cy = (cell.id / size_x) as f32;

        let x = cx * CELLSIZE - offset_x; // the cells screen coordinates
        let y = -cy * CELLSIZE + offset_y;

        commands
            .spawn((
                Sprite {
                    color: sprite_color,
                    custom_size: Some(Vec2::splat(CELLSIZE)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(x, y, 0.0)),
            ))
            .with_children(|parent| {
                if cell.cell_type == CellType::Obstacle || cell.cell_type == CellType::Exit {
                    return;
                }
                let valid_ways: Vec<(Direction, usize)> = cell.valid_ways().collect();
                for (way, id) in valid_ways {
                    let (dx, dy) = match way {
                        Direction::UP => (0.0, 1.0),
                        Direction::RIGHT => (1.0, 0.0),
                        Direction::DOWN => (0.0, -1.0),
                        Direction::LEFT => (-1.0, 0.0),
                    };
                    let x = dx * CELLSIZE * 0.3;
                    let y = dy * CELLSIZE * 0.3;

                    parent.spawn((
                        Text2d::new(id.to_string()),
                        fira.clone(),
                        Transform::from_translation(Vec3::new(x, y, 1.0)),
                    ));
                }
                parent.spawn((
                    Text2d::new(cell.id.to_string()),
                    fira.clone(),
                    TextColor(Color::hsl(200.0, 0.9, 0.1)),
                    Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                ));
            });
    }
}

fn change_resolution(mut window: Single<&mut Window>, grid: Res<Grid>) {
    let (x, y) = grid.size;
    let res = Vec2::new((x as f32 * CELLSIZE).round(), (y as f32 * CELLSIZE).round());
    window.resolution.set(res.x, res.y);
}
