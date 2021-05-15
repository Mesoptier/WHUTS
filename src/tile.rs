use crate::rotation_matrices::ROTATION_MATRICES_2D;

pub type Coord<const N: usize> = [usize; N];


#[derive(Debug, Clone)]
pub struct Tile<const N: usize> {
    pub coords: Vec<Coord<N>>,
}

impl<const N: usize> Tile<N> {
    pub fn new(mut coords: Vec<Coord<N>>) -> Tile<N> {
        // Make sure the coordinates are in sorted order, so tiles can easily be de-duplicated
        coords.sort();

        Tile { coords }
    }
}

impl Tile<2> {
    pub fn rotations(&self) -> Vec<Tile<2>> {
        ROTATION_MATRICES_2D.iter()
            .map(|mat| {
                let mut new_coords = vec![];

                for &coord in &self.coords {
                    let mut new_coord = [0; 2];
                    for i in 0..2 {
                        for j in 0..2 {
                            new_coord[i] += coord[j] as i8 * mat[i][j];
                        }
                    }
                    new_coords.push(new_coord);
                }

                return Tile::new(normalize_negative_coords(new_coords));
            })
            .collect()
    }
}

fn normalize_negative_coords<const N: usize>(coords: Vec<[i8; N]>) -> Vec<Coord<N>> {
    let mut lowest = [i8::MAX; N];
    for &coord in &coords {
        for i in 0..N {
            lowest[i] = std::cmp::min(lowest[i], coord[i]);
        }
    }

    coords.iter()
        .map(|coord| {
            let mut normalized_coord: Coord<N> = [0; N];
            for i in 0..N {
                normalized_coord[i] = (coord[i] - lowest[i]) as usize;
            }
            normalized_coord
        })
        .collect()
}