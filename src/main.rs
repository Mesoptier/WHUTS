use std::collections::HashSet;
use std::iter::FromIterator;

const TILE_SIZE: usize = 2;

type Tile = [[bool; TILE_SIZE]; TILE_SIZE];

fn main() {
    // Create the base tile
    let mut tile: Tile = [[false; TILE_SIZE]; TILE_SIZE];
    tile[0][0] = true;
    tile[0][1] = true;
    tile[1][0] = true;
    let tile = tile;

    // Generate all orientations of the given tile
    let mut tile_orientations: HashSet<Tile> = HashSet::new();
    for &fx in &[false, true] {
        for &fy in &[false, true] {
            let mut new_tile = tile;

            if fx {
                new_tile.reverse();
            }
            if fy {
                for x in 0..TILE_SIZE {
                    new_tile[x].reverse();
                }
            }

            // print_tile(&new_tile);
            // println!();

            tile_orientations.insert(new_tile);
        }
    }

    let size = (9, 9);

    let mat = WhutsMatrix::new(size, Vec::from_iter(tile_orientations));
    let mut solver = dlx::Solver::new(mat.ncols(), mat);
    let mut solutions = Solutions { size, solved: false };
    solver.solve(vec![], &mut solutions);
    if !solutions.solved {
        println!("No solution");
    }
}

struct WhutsMatrix {
    size: (usize, usize),
    tiles: Vec<Tile>,

    x: usize,
    y: usize,
    t: usize,
}

impl WhutsMatrix {
    fn new(size: (usize, usize), tiles: Vec<Tile>) -> WhutsMatrix {
        WhutsMatrix {
            size,
            tiles,

            t: 0,
            x: 0,
            y: 0,
        }
    }

    fn ncols(&self) -> usize {
        return self.size.0 * self.size.1;
    }
}

impl Iterator for WhutsMatrix {
    type Item = dlx::Row;

    fn next(&mut self) -> Option<Self::Item> {
        if self.t >= self.tiles.len() {
            return None;
        }

        let (size_x, size_y) = self.size;

        // === MAKE ROW
        let mut row = vec![];
        let tile = self.tiles[self.t];

        // Loop over each cell in the tile box
        // Compute coordinates in wrapped space ((self.x, self.y) + (dx, dy)) % self.size = (cx, cy)
        for dx in 0..TILE_SIZE {
            let cx = (self.x + dx) % size_x;
            for dy in 0..TILE_SIZE {
                let cy = (self.y + dy) % size_y;

                if tile[dx][dy] {
                    let cell_index = self.size.0 * cy + cx;
                    row.push(cell_index);
                }
            }
        }

        // === INCREMENT
        self.y += 1;
        if self.y == size_y {
            self.y = 0;
            self.x += 1;
            if self.x == size_x {
                self.x = 0;
                self.t += 1;
            }
        }

        return Some(row);
    }
}

struct Solutions {
    size: (usize, usize),
    solved: bool,
}

impl dlx::Solutions for Solutions {
    fn push(&mut self, sol: dlx::Solution) -> bool {
        self.solved = true;

        println!("Found a solution!");

        let (size_x, size_y) = self.size;
        let mut grid = vec![0; size_x * size_y];

        for (id, row) in sol.enumerate() {
            for cell_index in row {
                grid[cell_index] = id;
            }
        }

        for (cell_index, id) in grid.iter().enumerate() {
            if cell_index != 0 && cell_index % size_x == 0 {
                println!();
            }
            print!("[{:3}]", id);
        }

        // Stop after finding one solution
        false
    }
}

fn print_tile(tile: &Tile) {
    for x in 0..TILE_SIZE {
        for y in 0..TILE_SIZE {
            match tile[x][y] {
                true => print!("X"),
                false => print!("."),
            }
        }
        println!();
    }
}