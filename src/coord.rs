pub type Coord<const N: usize> = [usize; N];

pub fn coord_to_index<const N: usize>(coord: Coord<N>, space_size: Coord<N>) -> usize {
    let mut cell_index = 0;
    for i in 0..N {
        let mut x = coord[i];
        for j in 0..i {
            x *= space_size[j];
        }
        cell_index += x;
    }
    cell_index
}

pub fn index_to_coord<const N: usize>(index: usize, space_size: Coord<N>) -> Coord<N> {
    let mut coord = [0; N];

    for i in 0..N {
        let mut x = index;
        for j in 0..i {
            x /= space_size[j];
        }
        coord[i] = x % space_size[i];
    }

    coord
}

#[cfg(test)]
mod tests {
    use crate::coord::{index_to_coord, coord_to_index};

    #[test]
    fn test() {
        let space_size = [3, 5, 7];
        let max_index: usize = space_size.iter().product();

        for index in 0..max_index {
            assert_eq!(coord_to_index(index_to_coord(index, space_size), space_size), index);
        }
    }
}