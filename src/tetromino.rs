use rand::{
    distributions::{Distribution, Standard},
    random, Rng,
};

const MATRIX_SIZE: usize = 4;

type Matrix = [[bool; MATRIX_SIZE]; MATRIX_SIZE];
/// Matrices are based on the [Super Rotation System](https://tetris.wiki/Super_Rotation_System)

const fn l(orientation: &Orientation) -> Matrix {
    match orientation {
        Orientation::Up => [
            [false, false, true, false],
            [true, true, true, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        Orientation::Right => [
            [false, true, false, false],
            [false, true, false, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
        Orientation::Down => [
            [false, false, false, false],
            [true, true, true, false],
            [true, false, false, false],
            [false, false, false, false],
        ],
        Orientation::Left => [
            [true, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
    }
}

const fn j(orientation: &Orientation) -> Matrix {
    match orientation {
        Orientation::Up => [
            [true, false, false, false],
            [true, true, true, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        Orientation::Right => [
            [false, true, true, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
        Orientation::Down => [
            [false, false, false, false],
            [true, true, true, false],
            [false, false, true, false],
            [false, false, false, false],
        ],
        Orientation::Left => [
            [false, true, false, false],
            [false, true, false, false],
            [true, true, false, false],
            [false, false, false, false],
        ],
    }
}

const fn o(_orientation: &Orientation) -> Matrix {
    [
        [false, false, false, false],
        [false, true, true, false],
        [false, true, true, false],
        [false, false, false, false],
    ]
}

const fn i(orientation: &Orientation) -> Matrix {
    match orientation {
        Orientation::Up => [
            [false, false, false, false],
            [true, true, true, true],
            [false, false, false, false],
            [false, false, false, false],
        ],
        Orientation::Right => [
            [false, false, true, false],
            [false, false, true, false],
            [false, false, true, false],
            [false, false, true, false],
        ],
        Orientation::Down => [
            [false, false, false, false],
            [false, false, false, false],
            [true, true, true, true],
            [false, false, false, false],
        ],
        Orientation::Left => [
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
        ],
    }
}

const fn t(orientation: &Orientation) -> Matrix {
    match orientation {
        Orientation::Up => [
            [false, true, false, false],
            [true, true, true, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        Orientation::Right => [
            [false, true, false, false],
            [false, true, true, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
        Orientation::Down => [
            [false, false, false, false],
            [true, true, true, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
        Orientation::Left => [
            [false, true, false, false],
            [true, true, false, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
    }
}

const fn z(orientation: &Orientation) -> Matrix {
    match orientation {
        Orientation::Up => [
            [true, true, false, false],
            [false, true, true, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        Orientation::Right => [
            [false, false, true, false],
            [false, true, true, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
        Orientation::Down => [
            [false, false, false, false],
            [true, true, false, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
        Orientation::Left => [
            [true, false, false, false],
            [true, true, false, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
    }
}

const fn s(orientation: &Orientation) -> Matrix {
    match orientation {
        Orientation::Up => [
            [false, true, true, false],
            [true, true, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        Orientation::Right => [
            [false, true, false, false],
            [false, true, true, false],
            [false, false, true, false],
            [false, false, false, false],
        ],
        Orientation::Down => [
            [false, false, false, false],
            [false, true, true, false],
            [true, true, false, false],
            [false, false, false, false],
        ],
        Orientation::Left => [
            [true, false, false, false],
            [true, true, false, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
    }
}

enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

pub struct Tetromino {
    rotations: fn(orientation: &Orientation) -> Matrix,
    orientation: Orientation,
}

impl Tetromino {
    fn new(rotations: fn(orientation: &Orientation) -> Matrix) -> Self {
        Tetromino {
            rotations,
            orientation: Orientation::Up,
        }
    }
    pub fn random() -> Self {
        random()
    }
    pub fn rotate_clockwise(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        };
    }
    pub fn rotate_counterclockwise(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Up => Orientation::Left,
            Orientation::Right => Orientation::Up,
            Orientation::Down => Orientation::Right,
            Orientation::Left => Orientation::Down,
        };
    }
    pub fn matrix(&self) -> Matrix {
        (self.rotations)(&self.orientation)
    }
}

impl Distribution<Tetromino> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tetromino {
        match rng.gen_range(0..7) {
            0 => Tetromino::new(l),
            1 => Tetromino::new(j),
            2 => Tetromino::new(o),
            3 => Tetromino::new(i),
            4 => Tetromino::new(t),
            5 => Tetromino::new(z),
            6 => Tetromino::new(s),
            _ => unreachable!(),
        }
    }
}
