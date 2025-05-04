use bevy::prelude::*;

pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Resource, Debug, PartialEq, Copy, Clone)]
pub enum CellType {
    Empty,
    Obstacle,
    Portal(usize),
    Exit,
}

#[derive(Resource, Debug, Copy, Clone)]
pub struct GridCell {
    pub id: usize,
    pub cell_type: CellType,
    pub ways: (Option<usize>, Option<usize>, Option<usize>, Option<usize>),
}

#[derive(Resource, Debug)]
pub struct Grid {
    pub size: (usize, usize),
    pub cells: Vec<GridCell>,
    pub player_position: Option<(usize, usize)>,
}

// helper for wrapping around coordinates in the grid
fn wrap(value: isize, max: usize) -> usize {
    value.rem_euclid(max as isize) as usize
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            cells: Vec::new(),
            size: (0, 0),
            player_position: None,
        }
    }
    // get grid cell by (col, row)
    pub fn get(&self, (col, row): (isize, isize)) -> Result<GridCell> {
        let width = self.size.0;
        let height = self.size.1;

        if self.cells.is_empty() {
            return Err("Grid size is zero".into());
        }

        let wrapped_col = wrap(col, width);
        let wrapped_row = wrap(row, height);

        let index = wrapped_row * width + wrapped_col;

        let cell = self.cells.get(index).copied().ok_or("Cell not found")?; // Return GridCell, not &GridCell

        Ok(cell)
    }

    // populate the grids cells with information about which cell you will end up
    // if you walk in a certain direction (recursively call 'walk' until you hit an obstacle)
    // also: follow portals if encountered
    pub fn walk(
        &self,
        direction: Direction,
        cell: usize, // Use usize (index)
        distance: usize,
    ) -> Option<usize> {
        let (width, _height) = self.size;
        let (col, row) = (
            cell as isize % width as isize,
            cell as isize / width as isize,
        );
        let next_cell = match direction {
            Direction::UP => self.get((col, row - 1)),
            Direction::RIGHT => self.get((col + 1, row)),
            Direction::DOWN => self.get((col, row + 1)),
            Direction::LEFT => self.get((col - 1, row)),
        }
        .expect("Failed to get next cell");

        return match next_cell.cell_type {
            CellType::Obstacle => {
                if distance == 0 {
                    return None;
                }
                return Some(cell);
            }
            CellType::Exit => Some(next_cell.id),
            CellType::Portal(target) => Some(self.portal_destination_index(target)),
            CellType::Empty => {
                if distance > 20 {
                    println!("{} ", cell);
                    return None;
                } else {
                    return self.walk(direction, next_cell.id, distance + 1);
                }
            }
        };
    }

    // find the destination of a portal
    pub fn portal_destination_index(&self, destination_id: usize) -> usize {
        let mut portals: Vec<usize> = Vec::new();
        for i in 0..self.cells.len() {
            if matches!(self.cells[i].cell_type, CellType::Portal(_)) {
                portals.push(i); // or: self.cells[i].id ; same thing
            }
        }
        return portals[destination_id];
    }
}
