use crate::grid::{CellType, Grid};
use bevy::prelude::*;

use std::sync::Mutex;

#[derive(Resource, Debug)]
pub enum Strategy {
    NoDuplicateEdges,
    NoDuplicateNodes,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct PlayerStart(pub (isize, isize));

lazy_static::lazy_static! {
    static ref LONGEST_PATH: Mutex<Vec<(usize, usize)>> = Mutex::new(Vec::new());
}

trait ChillyMoves {
    fn contains_edge(&self, node: usize) -> bool;
    fn contains_node(&self, node: usize) -> bool;
    fn commands(&self) -> String;
}

impl ChillyMoves for Vec<(usize, usize)> {
    fn contains_edge(&self, node: usize) -> bool {
        if self.len() < 1 {
            return false;
        }
        let last_cell_id = self.last().map(|&(id, _)| id).unwrap();

        self.windows(2).any(|window| {
            window[0].0 == last_cell_id && window[1].0 == node
        })
    }
    fn contains_node(&self, node: usize) -> bool {
        self.iter().any(|&(id, _)| id == node)
    }
    fn commands(&self) -> String {
        let directions = ['U', 'R', 'D', 'L'];
            self.iter()
                .skip(1) // Chilly's starting point. No direction needed
                .map(|(_, dir_index)| directions.get(*dir_index).unwrap_or(&'?'))
                .collect::<String>()
    }
}

pub fn solve(grid: Res<Grid>, strategy: Res<Strategy>, player_start: Res<PlayerStart>) {
    let first_cell_id = grid.get(player_start.0).unwrap().id;
    unsafe {
        follow_path(&grid, &*strategy, first_cell_id, vec![(first_cell_id, 0 as usize)].as_mut());
    }
    println!("Longest Command Sequence:\n{}", LONGEST_PATH.lock().unwrap().clone().commands());
}

fn follow_path(grid: &Grid, strategy: &Strategy, cell_id: usize, acc: &mut Vec<(usize, usize)>) {
    let (up, right, down, left) = grid.cells[cell_id].ways;
    for (direction_index, next_cell_id) in [up, right, down, left].iter().enumerate() {
        if let Some(id) = next_cell_id {
            match strategy {
                Strategy::NoDuplicateEdges => {
                    if acc.contains_edge(*id) {
                        continue;
                    }
                }
                Strategy::NoDuplicateNodes => {
                    if acc.contains_node(*id) {
                        continue;
                    }
                }
            }
            if grid.cells[*id].cell_type == CellType::Exit {
                acc.push((*id, direction_index));

                let mut longest_path = LONGEST_PATH.lock().unwrap();
                if acc.len() > longest_path.len() {
                    longest_path.clear();
                    longest_path.extend(acc.iter().copied());
                    println!("Found : {}", longest_path.commands());
                }

                continue;
            }
            let mut new_acc = acc.clone();
            new_acc.push((*id, direction_index));
            follow_path(grid, strategy, *id, new_acc.as_mut());
        }
    }
}

