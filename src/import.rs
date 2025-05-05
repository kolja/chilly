use crate::grid::{CellType, Direction, Grid, GridCell};
use bevy::prelude::*;
use std::fs;

#[derive(Resource, Clone, Copy)]
pub enum Level {
    MiniEiersuche,
    KeinHaltZweimal,
    KeineDoppeltenWege,
}

impl Level {
    fn filename(&self) -> &'static str {
        match self {
            Level::MiniEiersuche => "assets/mini-eiersuche.txt",
            Level::KeinHaltZweimal => "assets/kein-halt-zweimal.txt",
            Level::KeineDoppeltenWege => "assets/keine-doppelten-wege.txt",
        }
    }
}

pub struct Import {
    level: Level,
}

impl Import {
    pub fn new(level: Level) -> Self {
        Self { level }
    }
}

impl Plugin for Import {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.level)
            .add_systems(Startup, (load_level, find_ways).chain());
    }
}

fn load_level(level: Res<Level>, mut grid: ResMut<Grid>) {
    let contents = fs::read_to_string(level.filename());

    match contents {
        Ok(contents) => {
            for (row_index, line) in contents.lines().enumerate() {
                for (col_index, c) in line.chars().enumerate() {
                    let cell_type = match c {
                        '#' => CellType::Obstacle,
                        '0'..='9' => {
                            let digit = c.to_digit(10).unwrap_or(0) as usize;
                            CellType::Portal(digit)
                        }
                        'x' => CellType::Exit,
                        _ => CellType::Empty,
                    };
                    grid.cells.push(GridCell {
                        id: row_index * line.len() + col_index,
                        cell_type,
                        ways: (None, None, None, None),
                    });
                }
            }
            let rows = contents.lines().count();
            let cols = grid.cells.len() / rows;
            grid.size = (cols, rows);
        }
        Err(e) => {
            error!("Failed to read level: {}", e);
        }
    }
}

fn find_ways(mut grid: ResMut<Grid>) {
    let size = grid.cells.len();
    for i in 0..size {
        grid.cells[i].ways = (
            grid.walk(Direction::UP, grid.cells[i].id, 0),
            grid.walk(Direction::RIGHT, grid.cells[i].id, 0),
            grid.walk(Direction::DOWN, grid.cells[i].id, 0),
            grid.walk(Direction::LEFT, grid.cells[i].id, 0),
        );
    }
}

