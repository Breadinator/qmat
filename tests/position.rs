use qmat::position::Position;

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
mod disp {
    use qmat::position::Position;

    const POS: Position = Position(3, 4);

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", POS), "Position { 3, 4 }");
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", POS), "(3, 4)");
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
    fn up_n() {
        assert_eq!(POS.up_n(2), Position(1, 3));
    }

    #[test]
    fn down() {
        assert_eq!(POS.down(), Position(4, 3));
    }
    #[test]
    fn down_n() {
        assert_eq!(POS.down_n(7), Position(10, 3));
    }

    #[test]
    fn left() {
        assert_eq!(POS.left(), Position(3, 2));
    }
    #[test]
    fn left_n() {
        assert_eq!(POS.left_n(3), Position(3, 0));
    }

    #[test]
    fn right() {
        assert_eq!(POS.right(), Position(3, 4));
    }
    #[test]
    fn right_n() {
        assert_eq!(POS.right_n(13), Position(3, 16));
    }
}

#[test]
fn into_and_from() {
    let a: [usize; 2] = [12, 4];
    let b: Position = a.into();
    assert_eq!(b.0, 12);
    assert_eq!(b.1, 4);
    let c: [usize; 2] = b.into();
    assert_eq!(a, c);
}
