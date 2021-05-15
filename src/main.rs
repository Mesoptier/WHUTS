use std::collections::HashSet;
use std::iter::FromIterator;

use crate::tile::{Tile, Coord};

mod tile;
mod rotation_matrices;

fn main() {
    let mut tile: Tile<2> = Tile::new(vec![[0, 0], [0, 1], [1, 0]]);

    let space_size = [3, 3];

    let mat = WhutsMatrix::new(space_size, tile.rotations());
    let mut solver = dlx::Solver::new(mat.ncols(), mat);
    let mut solutions = Solutions { solved: false };
    solver.solve(vec![], &mut solutions);
    if !solutions.solved {
        println!("No solution");
    }
}

struct WhutsMatrix<const N: usize> {
    size: Coord<N>,
    tiles: Vec<Tile<N>>,

    t: usize,
    pos: Coord<N>,
}

impl<const N: usize> WhutsMatrix<N> {
    fn new(size: Coord<N>, tiles: Vec<Tile<N>>) -> WhutsMatrix<N> {
        WhutsMatrix {
            size,
            tiles,

            t: 0,
            pos: [0; N],
        }
    }

    fn ncols(&self) -> usize {
        let mut ncols = 1;
        for i in 0..N {
            ncols *= self.size[i];
        }
        ncols
    }
}

impl<const N: usize> Iterator for WhutsMatrix<N> {
    type Item = dlx::Row;

    fn next(&mut self) -> Option<Self::Item> {
        if self.t >= self.tiles.len() {
            return None;
        }

        // === MAKE ROW
        let mut row = vec![];
        let tile = &self.tiles[self.t];

        for tile_coord in &tile.coords {
            // cell_coord = (self.pos + tile_coord) % self.size // (element-wise)
            let mut cell_coord = [0; N];
            for i in 0..N {
                cell_coord[i] = (self.pos[i] + tile_coord[i]) % self.size[i];
            }

            // ND coord to 1D index
            let mut cell_index = 0;
            for i in 0..N {
                let mut x = cell_coord[i];
                for j in 0..i {
                    x *= self.size[j];
                }
                cell_index += x;
            }

            row.push(cell_index);
        }

        // === INCREMENT
        for i in 0..=N {
            if i == N {
                self.t += 1;
                break;
            }

            self.pos[i] += 1;

            if self.pos[i] != self.size[i] {
                break;
            }
            self.pos[i] = 0;
        }

        return Some(row);
    }
}

struct Solutions {
    solved: bool,
}

impl dlx::Solutions for Solutions {
    fn push(&mut self, sol: dlx::Solution) -> bool {
        self.solved = true;

        println!("Found a solution!");

        for row in sol {
            println!("{:?}", row);
        }

        // Stop after finding one solution
        false
    }
}
