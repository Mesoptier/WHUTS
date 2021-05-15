use std::collections::HashSet;

const TILE_SIZE: usize = 2;

type Tile = [[bool; TILE_SIZE]; TILE_SIZE];

fn main() {
    let mut tile: Tile = [[false; TILE_SIZE]; TILE_SIZE];
    tile[0][0] = true;
    tile[0][1] = true;
    tile[1][0] = true;

    print_tile(&tile);

    let mut tile_orientations: HashSet<Tile> = HashSet::new();
    tile_orientations.insert(tile);


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