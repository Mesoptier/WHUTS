use crate::rotation_matrices::{ROTATION_MATRICES_2D, ROTATION_MATRICES_3D};
use crate::coord::Coord;


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Tile<const N: usize> {
    pub coords: Vec<Coord<N>>,
}

impl<const N: usize> Tile<N> {
    pub fn new(mut coords: Vec<Coord<N>>) -> Tile<N> {
        // Make sure the coordinates are in sorted order, so tiles can easily be de-duplicated
        coords.sort();

        Tile { coords }
    }

    fn rotations_internal(&self, matrices: &[[[i8; N]; N]]) -> Vec<Tile<N>> {
        matrices.iter()
            .map(|mat| {
                let mut new_coords = vec![];

                for &coord in &self.coords {
                    let mut new_coord = [0; N];
                    for i in 0..N {
                        for j in 0..N {
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

impl Tile<2> {
    pub fn rotations(&self) -> Vec<Tile<2>> {
        self.rotations_internal(&ROTATION_MATRICES_2D)
    }
}

impl Tile<3> {
    pub fn rotations(&self) -> Vec<Tile<3>> {
        self.rotations_internal(&ROTATION_MATRICES_3D)
    }
}

pub fn normalize_negative_coords<const N: usize>(coords: Vec<[i8; N]>) -> Vec<Coord<N>> {
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