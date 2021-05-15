use std::collections::HashSet;

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