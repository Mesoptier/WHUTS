use crate::tile::{Tile, Coord, normalize_negative_coords};

mod tile;
mod rotation_matrices;
mod data;

const N: usize = 3;
const MAX_SPACE_SIZE: usize = 5;

fn main() {
    for &coords in &data::TILES {
        let tile: Tile<N> = Tile::new(normalize_negative_coords(Vec::from(coords)));

        println!("{:?}", tile);

        // Get and de-dupe rotations
        let mut rotations = tile.rotations();
        rotations.sort();
        rotations.dedup();

        find_tiling(rotations);
    }
}

fn find_tiling(tiles: Vec<Tile<N>>) -> bool {
    for x in 1..MAX_SPACE_SIZE {
        for y in 1..MAX_SPACE_SIZE {
            for z in 1..MAX_SPACE_SIZE {
                let space_size = [x, y, z];
                if find_tiling_in_space(tiles.clone(), space_size) {
                    println!("Wrapped space size: {:?}", space_size);
                    return true;
                }
            }
        }
    }
    return false;
}

fn find_tiling_in_space(tiles: Vec<Tile<N>>, space_size: Coord<N>) -> bool {
    let num_cells = space_size.iter().product::<usize>();
    if num_cells % tiles.first().unwrap().coords.len() != 0 {
        return false;
    }

    // Solve using DLX
    let mat = WhutsMatrix::new(space_size, tiles);
    let mut solver = dlx::Solver::new(mat.ncols(), mat);
    let mut solutions = Solutions { solved: false };
    solver.solve(vec![], &mut solutions);

    return solutions.solved;
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
        self.size.iter().product()
    }
}

impl<const N: usize> Iterator for WhutsMatrix<N> {
    type Item = dlx::Row;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.t >= self.tiles.len() {
                return None;
            }

            // === MAKE ROW
            let mut row = vec![];
            let tile = &self.tiles[self.t];

            let mut is_valid = true;

            for tile_coord in &tile.coords {
                // cell_coord = (self.pos + tile_coord) % self.size // (element-wise)
                let mut cell_coord = [0; N];
                for i in 0..N {
                    cell_coord[i] = (self.pos[i] + tile_coord[i]) % self.size[i];
                }

                // println!("{:?} = ({:?} + {:?}) % {:?}", cell_coord, self.pos, tile_coord, self.size);

                // ND coord to 1D index
                let mut cell_index = 0;
                for i in 0..N {
                    let mut x = cell_coord[i];
                    for j in 0..i {
                        x *= self.size[j];
                    }
                    // println!("{}", x);
                    cell_index += x;
                }

                // println!("{:?} -> {}", cell_coord, cell_index);

                if row.contains(&cell_index) {
                    // Tile intersects itself, so this row would not be valid
                    is_valid = false;
                    break;
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

            if is_valid {
                return Some(row);
            }
        };
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
