#[test]
fn position_indexing() {
    use qmat::{matrix, position::Position};

    let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let mat = matrix!(3, 4, data);

    for i in 0..mat.rows() {
        for j in 0..mat.cols() {
            let pos = Position(i, j);
            assert_eq!(mat[[i, j]], mat[pos]);
        }
    }
}

#[cfg(test)]
mod moving {
    use qmat::position::Position;

    const POS: Position = Position(3, 3);

    #[test]
    fn up() {
        assert_eq!(POS.up(), Position(2, 3));
    }

    #[test]
    fn down() {
        assert_eq!(POS.down(), Position(4, 3));
    }

    #[test]
    fn left() {
        assert_eq!(POS.left(), Position(3, 2));
    }

    #[test]
    fn right() {
        assert_eq!(POS.right(), Position(3, 4));
    }
}
