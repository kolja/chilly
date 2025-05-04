
use bevy::prelude::*;
use crate::grid::{CellType, Grid, GridCell};

use std::sync::Mutex;
use lazy_static::lazy_static;

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

pub fn solve(
    grid: Res<Grid>,
    strategy: Res<Strategy>,
    player_start: Res<PlayerStart>
) {
    let first_cell_id = grid.get(player_start.0).unwrap().id;
    follow_path(&grid, first_cell_id, vec![first_cell_id]);

    println!("Longest path: {:?}", *LONGEST_PATH.lock().unwrap());
}

fn follow_path(grid: &Grid, cell_id: usize, acc: Vec<usize>) {
    let cell = grid.cells[cell_id];
    let (way1, way2, way3, way4) = cell.ways;
    // wip
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
        print!("[{} {} {} {}]", way1.unwrap_or(0), way2.unwrap_or(0), way3.unwrap_or(0), way4.unwrap_or(0));
        println!();
    }
}
