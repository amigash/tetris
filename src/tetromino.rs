/// <https://tetris.wiki/Super_Rotation_System>
type Matrix<const N: usize> = [[bool; N]; N];

const L: [Matrix<3>; 4] = [
    [
        [false, false, true],
        [true, true, true],
        [false, false, false],
    ],
    [
        [false, true, false],
        [false, true, false],
        [false, true, true],
    ],
    [
        [false, false, false],
        [true, true, true],
        [true, false, false],
    ],
    [
        [true, true, false],
        [false, true, false],
        [false, true, false],
    ],
];

const J: [Matrix<3>; 4] = [
    [
        [true, false, false],
        [true, true, true],
        [false, false, false],
    ],
    [
        [false, true, true],
        [false, true, false],
        [false, true, false],
    ],
    [
        [false, false, false],
        [true, true, true],
        [false, false, true],
    ],
    [
        [false, true, false],
        [false, true, false],
        [true, true, false],
    ],
];
const O: Matrix<2> = [[true, true], [true, true]];

const I: [Matrix<4>; 4] = [
    [
        [false, false, false, false],
        [true, true, true, true],
        [false, false, false, false],
        [false, false, false, false],
    ],
    [
        [false, false, true, false],
        [false, false, true, false],
        [false, false, true, false],
        [false, false, true, false],
    ],
    [
        [false, false, false, false],
        [false, false, false, false],
        [true, true, true, true],
        [false, false, false, false],
    ],
    [
        [false, true, false, false],
        [false, true, false, false],
        [false, true, false, false],
        [false, true, false, false],
    ],
];

const T: [Matrix<3>; 4] = [
    [
        [false, true, false],
        [true, true, true],
        [false, false, false],
    ],
    [
        [false, true, false],
        [false, true, true],
        [false, true, false],
    ],
    [
        [false, false, false],
        [true, true, true],
        [false, true, false],
    ],
    [
        [false, true, false],
        [true, true, false],
        [false, true, false],
    ],
];

const Z: [Matrix<3>; 4] = [
    [
        [true, true, false],
        [false, true, true],
        [false, false, false],
    ],
    [
        [false, false, true],
        [false, true, true],
        [false, true, false],
    ],
    [
        [false, false, false],
        [true, true, false],
        [false, true, true],
    ],
    [
        [false, true, false],
        [true, true, false],
        [true, false, false],
    ],
];

const S: [Matrix<3>; 4] = [
    [
        [false, true, true],
        [true, true, false],
        [false, false, false],
    ],
    [
        [false, true, false],
        [false, true, true],
        [false, false, true],
    ],
    [
        [false, false, false],
        [false, true, true],
        [true, true, false],
    ],
    [
        [true, false, false],
        [true, true, false],
        [false, true, false],
    ],
];
pub enum Tetromino {
    O { matrix: Matrix<2> },
    I { rotation: [Matrix<4>; 4] },
    L { rotation: [Matrix<3>; 4] },
    J { rotation: [Matrix<3>; 4] },
    T { rotation: [Matrix<3>; 4] },
    Z { rotation: [Matrix<3>; 4] },
    S { rotation: [Matrix<3>; 4] },
}

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
impl Distribution<Tetromino> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tetromino {
        match rng.gen_range(0..=6) {
            0 => Tetromino::O { matrix: O },
            1 => Tetromino::I { rotation: I },
            2 => Tetromino::L { rotation: L },
            3 => Tetromino::J { rotation: J },
            4 => Tetromino::T { rotation: T },
            5 => Tetromino::Z { rotation: Z },
            6 => Tetromino::S { rotation: S },
            _ => unreachable!(),
        }
    }
}
