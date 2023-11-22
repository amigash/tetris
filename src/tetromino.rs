/// https://tetris.wiki/Super_Rotation_System
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
const O: Matrix<2> = [
        [true, true],
        [true, true],
];

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
    O {
        matrix: Matrix<2>
    },
    I {
        rotation: [Matrix<4>; 4]
    },
    L {
        rotation: [Matrix<3>; 4]
    },
    J {
        rotation: [Matrix<3>; 4]
    },
    T {
        rotation: [Matrix<3>; 4]
    },
    Z {
        rotation: [Matrix<3>; 4]
    },
    S {
        rotation: [Matrix<3>; 4]
    },
}

impl Tetromino {
    pub fn initialize() -> [Tetromino; 7] {
        [
            Tetromino::O {
                matrix: O
            },
            Tetromino::I {
                rotation: I
            },
            Tetromino::L {
                rotation: L
            },
            Tetromino::J {
                rotation: J
            },
            Tetromino::T {
                rotation: T
            },
            Tetromino::Z {
                rotation: Z
            },
            Tetromino::S {
                rotation: S
            },
        ]
    }
}
