use crate::tile::{Tile, normalize_negative_coords};
use crate::coord::{Coord, coord_to_index, index_to_coord};
use itertools::Itertools;
use std::convert::TryInto;

mod tile;
mod rotation_matrices;
mod data;
mod coord;

const N: usize = 3;
const MAX_SPACE_SIZE: usize = 8;

type Tiling<const N: usize> = Vec<Vec<Coord<N>>>;

fn main() {
    for (index, &coords) in data::TILES.iter().enumerate() {
        let tile: Tile<N> = Tile::new(normalize_negative_coords(Vec::from(coords)));

        println!("{:=^88}", format!("[ UNFOLDING #{} ]", index + 1));
        println!("Unfolding coords:\n{:?}", tile.coords);
        println!();

        // Get and de-dupe rotations
        let mut rotations = tile.rotations();
        rotations.sort();
        rotations.dedup();

        match find_tiling(rotations) {
            Some((tiling, space_size)) => {
                println!("FOUND TILING!");
                println!("Dimensions of the wrapped space (i.e. hypertorus): {:?}", space_size);
                println!("Tiling (each line is a single instance of the unfolding):");
                for tile in tiling {
                    println!("{:?}", tile);
                }
            }
            None => {
                println!("NO TILING FOUND!")
            }
        }
        println!();
    }
}

fn find_tiling(tiles: Vec<Tile<N>>) -> Option<(Tiling<N>, Coord<N>)> {
    let mut space_sizes: Vec<Coord<N>> = (1..MAX_SPACE_SIZE)
        .combinations_with_replacement(N)
        .map(|coord| coord.try_into().unwrap())
        .collect();

    // Prefer smaller space sizes first
    space_sizes.sort_unstable_by_key(|size| size.iter().product::<usize>());

    for space_size in space_sizes {
        // println!("(trying space size: {:?})", space_size);
        if let Some(tiling) = find_tiling_in_space(tiles.clone(), space_size) {
            return Some((tiling, space_size));
        }
    }
    return None;
}

fn find_tiling_in_space(tiles: Vec<Tile<N>>, space_size: Coord<N>) -> Option<Tiling<N>> {
    let num_cells = space_size.iter().product::<usize>();
    if num_cells % tiles.first().unwrap().coords.len() != 0 {
        return None;
    }

    // Solve using DLX
    let mat = WhutsMatrix::new(space_size, tiles);
    let mut solver = dlx::Solver::new(mat.ncols(), mat);
    let mut solutions = WhutsSolutions { space_size, solved: false, tiling: vec![] };
    solver.solve(vec![], &mut solutions);

    if !solutions.solved {
        return None;
    }
    return Some(solutions.tiling);
}

struct WhutsMatrix {
    size: Coord<N>,
    tiles: Vec<Tile<N>>,

    t: usize,
    pos: Coord<N>,
}

impl WhutsMatrix {
    fn new(size: Coord<N>, tiles: Vec<Tile<N>>) -> WhutsMatrix {
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

impl Iterator for WhutsMatrix {
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
                let cell_index = coord_to_index(cell_coord, self.size);

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

struct WhutsSolutions {
    space_size: Coord<N>,
    solved: bool,
    tiling: Tiling<N>,
}

impl dlx::Solutions for WhutsSolutions {
    fn push(&mut self, sol: dlx::Solution) -> bool {
        self.solved = true;

        for row in sol {
            let mut tile = vec![];
            for index in row {
                tile.push(index_to_coord(index, self.space_size));
            }
            tile.sort();
            self.tiling.push(tile);
        }

        self.tiling.sort();

        // Stop after finding one solution
        false
    }
}
