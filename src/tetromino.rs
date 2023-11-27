use rand::{
    distributions::{Distribution, Standard},
    random, Rng,
};

const fn l(orientation: Orientation) -> [[usize; 2]; 4] {
    match orientation {
        Orientation::Up => [[0, 2], [1, 0], [1, 1], [1, 2]],
        Orientation::Right => [[0, 1], [1, 1], [2, 1], [2, 2]],
        Orientation::Down => [[1, 0], [1, 1], [1, 2], [2, 0]],
        Orientation::Left => [[0, 0], [0, 1], [1, 1], [2, 1]],
    }
}

const fn j(orientation: Orientation) -> [[usize; 2]; 4] {
    match orientation {
        Orientation::Up => [[0, 0], [1, 0], [1, 1], [1, 2]],
        Orientation::Right => [[0, 1], [0, 2], [1, 1], [2, 1]],
        Orientation::Down => [[1, 0], [1, 1], [1, 2], [2, 2]],
        Orientation::Left => [[0, 1], [1, 1], [2, 0], [2, 1]],
    }
}

const fn o(_orientation: Orientation) -> [[usize; 2]; 4] {
    [[0, 1], [0, 2], [1, 1], [1, 2]]
}

const fn i(orientation: Orientation) -> [[usize; 2]; 4] {
    match orientation {
        Orientation::Up => [[1, 0], [1, 1], [1, 2], [1, 3]],
        Orientation::Right => [[0, 2], [1, 2], [2, 2], [3, 2]],
        Orientation::Down => [[2, 0], [2, 1], [2, 2], [2, 3]],
        Orientation::Left => [[0, 1], [1, 1], [2, 1], [3, 1]],
    }
}

const fn t(orientation: Orientation) -> [[usize; 2]; 4] {
    match orientation {
        Orientation::Up => [[0, 1], [1, 0], [1, 1], [1, 2]],
        Orientation::Right => [[0, 1], [1, 1], [1, 2], [2, 1]],
        Orientation::Down => [[1, 0], [1, 1], [1, 2], [2, 1]],
        Orientation::Left => [[0, 1], [1, 0], [1, 1], [2, 1]],
    }
}

const fn z(orientation: Orientation) -> [[usize; 2]; 4] {
    match orientation {
        Orientation::Up => [[0, 0], [0, 1], [1, 1], [1, 2]],
        Orientation::Right => [[0, 2], [1, 1], [1, 2], [2, 1]],
        Orientation::Down => [[1, 0], [1, 1], [2, 1], [2, 2]],
        Orientation::Left => [[0, 0], [1, 0], [1, 1], [2, 1]],
    }
}

const fn s(orientation: Orientation) -> [[usize; 2]; 4] {
    match orientation {
        Orientation::Up => [[0, 1], [0, 2], [1, 0], [1, 1]],
        Orientation::Right => [[0, 1], [1, 1], [1, 2], [2, 2]],
        Orientation::Down => [[1, 1], [1, 2], [2, 0], [2, 1]],
        Orientation::Left => [[0, 0], [1, 0], [1, 1], [2, 1]],
    }
}

#[derive(Copy, Clone)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone)]
pub struct Tetromino {
    rotations: fn(orientation: Orientation) -> [[usize; 2]; 4],
    orientation: Orientation,
    pub color: u32,
}

impl Tetromino {
    fn new(rotations: fn(orientation: Orientation) -> [[usize; 2]; 4], color: u32) -> Self {
        Tetromino {
            rotations,
            orientation: Orientation::Up,
            color,
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
    pub fn blocks(&self) -> [[usize; 2]; 4] {
        (self.rotations)(self.orientation)
    }
}

impl Distribution<Tetromino> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tetromino {
        match rng.gen_range(0..7) {
            0 => Tetromino::new(l, 0xC1_68_15),
            1 => Tetromino::new(j, 0x14_1B_CB),
            2 => Tetromino::new(o, 0xCB_CC_24),
            3 => Tetromino::new(i, 0x58_CC_CD),
            4 => Tetromino::new(t, 0x91_22_CB),
            5 => Tetromino::new(z, 0xBE_19_0F),
            6 => Tetromino::new(s, 0x53_CA_1F),
            _ => unreachable!(),
        }
    }
}
