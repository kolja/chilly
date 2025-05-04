use crate::grid::{CellType, Grid, GridCell};
use bevy::prelude::*;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Resource, Debug)]
pub enum Strategy {
    NoDuplicateEdges,
    NoDuplicateNodes,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct PlayerStart(pub (isize, isize));

lazy_static! {
    static ref LONGEST_PATH: Mutex<Vec<usize>> = Mutex::new(Vec::new());
}

trait ContainsEdge {
    fn contains_edge(&self, node: &usize) -> bool;
}

impl ContainsEdge for Vec<usize> {
    fn contains_edge(&self, node: &usize) -> bool {
        self.windows(2).any(|window| window[1] == *node)
    }
}

pub fn solve(grid: Res<Grid>, player_start: Res<PlayerStart>) {
    let first_cell_id = grid.get(player_start.0).unwrap().id;
    unsafe {
        follow_path(&grid, first_cell_id, vec![first_cell_id]);
    }

    println!("Longest path: {:?}", *LONGEST_PATH.lock().unwrap());
}

fn follow_path(grid: &Grid, cell_id: usize, acc: Vec<usize>) {
    let (up, right, down, left) = grid.cells[cell_id].ways;
    for next_cell_id in [up, right, down, left] {
        if let Some(id) = next_cell_id {
            if acc.contains_edge(&id) {
                continue;
            }
            if grid.cells[id].cell_type == CellType::Exit {
                if acc.len() > LONGEST_PATH.lock().unwrap().len() {
                    LONGEST_PATH.lock().unwrap().clear();
                    LONGEST_PATH.lock().unwrap().extend(acc.clone());
                }
                continue;
            }
            let mut new_acc = acc.clone();
            new_acc.push(id);
            follow_path(grid, id, new_acc);
        }
    }
}

struct Node {
    cell: usize,
    path: Vec<usize>,
}

pub fn print_grid(grid: Res<Grid>) {
    println!("-- Grid looks like this: --");
    for cell in &grid.cells {
        match cell.cell_type {
            CellType::Empty => print!(" "),
            CellType::Obstacle => print!("#"),
            CellType::Portal(num) => print!("{}", num),
            CellType::Exit => print!("x"),
        }
    }
    println!();
}

pub fn print_ways(grid: Res<Grid>) {
    println!("-- which node can you go to from any given node: --");
    for cell in &grid.cells {
        let (way1, way2, way3, way4) = cell.ways;
        print!(
            "[{} {} {} {}]",
            way1.unwrap_or(0),
            way2.unwrap_or(0),
            way3.unwrap_or(0),
            way4.unwrap_or(0)
        );
        println!();
    }
}
