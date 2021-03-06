pub const ROTATION_MATRICES_2D: [[[i8; 2]; 2]; 4] = [
    [
        [1, 0],
        [0, 1]
    ],
    [
        [0, 1],
        [-1, 0]
    ],
    [
        [-1, 0],
        [0, -1]
    ],
    [
        [0, -1],
        [1, 0]
    ],
];

pub const ROTATION_MATRICES_3D: [[[i8; 3]; 3]; 24] = [
    [
        [1, 0, 0],
        [0, 1, 0],
        [0, 0, 1],
    ],
    [
        [1, 0, 0],
        [0, 0, -1],
        [0, 1, 0],
    ],
    [
        [1, 0, 0],
        [0, -1, 0],
        [0, 0, -1],
    ],
    [
        [1, 0, 0],
        [0, 0, 1],
        [0, -1, 0],
    ],
    [
        [0, -1, 0],
        [1, 0, 0],
        [0, 0, 1],
    ],
    [
        [0, 0, 1],
        [1, 0, 0],
        [0, 1, 0],
    ],
    [
        [0, 1, 0],
        [1, 0, 0],
        [0, 0, -1],
    ],
    [
        [0, 0, -1],
        [1, 0, 0],
        [0, -1, 0],
    ],
    [
        [-1, 0, 0],
        [0, -1, 0],
        [0, 0, 1],
    ],
    [
        [-1, 0, 0],
        [0, 0, -1],
        [0, -1, 0],
    ],
    [
        [-1, 0, 0],
        [0, 1, 0],
        [0, 0, -1],
    ],
    [
        [-1, 0, 0],
        [0, 0, 1],
        [0, 1, 0],
    ],
    [
        [0, 1, 0],
        [-1, 0, 0],
        [0, 0, 1],
    ],
    [
        [0, 0, 1],
        [-1, 0, 0],
        [0, -1, 0],
    ],
    [
        [0, -1, 0],
        [-1, 0, 0],
        [0, 0, -1],
    ],
    [
        [0, 0, -1],
        [-1, 0, 0],
        [0, 1, 0],
    ],
    [
        [0, 0, -1],
        [0, 1, 0],
        [1, 0, 0],
    ],
    [
        [0, 1, 0],
        [0, 0, 1],
        [1, 0, 0],
    ],
    [
        [0, 0, 1],
        [0, -1, 0],
        [1, 0, 0],
    ],
    [
        [0, -1, 0],
        [0, 0, -1],
        [1, 0, 0],
    ],
    [
        [0, 0, -1],
        [0, -1, 0],
        [-1, 0, 0],
    ],
    [
        [0, -1, 0],
        [0, 0, 1],
        [-1, 0, 0],
    ],
    [
        [0, 0, 1],
        [0, 1, 0],
        [-1, 0, 0],
    ],
    [
        [0, 1, 0],
        [0, 0, -1],
        [-1, 0, 0],
    ],
];