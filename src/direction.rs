use num::{NumCast, cast};
use std::hash::Hash;
use std::ops;

pub type Pos = (isize, isize);

pub fn pos_from<T: NumCast, U: NumCast>(x: T, y: U) -> Pos {
    (cast(x).unwrap(), cast(y).unwrap())
}

pub fn pos_in_square_grid<T: NumCast + Copy>(x: T) -> impl Iterator<Item = Pos> {
    pos_in_grid(x, x)
}

pub fn pos_in_grid<T: NumCast, U: NumCast>(x: T, y: U) -> impl Iterator<Item = Pos> {
    let m: isize = cast(x).unwrap();
    let n: isize = cast(y).unwrap();
    (0..n).flat_map(move |y| (0..m).map(move |x| (x, y)))
}

pub fn pos_add(p1: &Pos, p2: &Pos) -> Pos {
    (p1.0 + p2.0, p1.1 + p2.1)
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub enum Direction {
    #[default]
    North,
    South,
    West,
    East,
}

pub static DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

impl Direction {
    pub fn from_char(c: char) -> Direction {
        match c {
            '^' | 'N' | 'U' => Direction::North,
            'v' | 'S' | 'D' => Direction::South,
            '<' | 'W' | 'L' => Direction::West,
            '>' | 'E' | 'R' => Direction::East,
            _ => panic!("Invalid direction {:?}", &c),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::West => '<',
            Direction::East => '>',
        }
    }

    pub fn to_char_udlr(&self) -> char {
        match self {
            Direction::North => 'U',
            Direction::South => 'D',
            Direction::West => 'L',
            Direction::East => 'R',
        }
    }

    pub fn to_char_nswe(&self) -> char {
        match self {
            Direction::North => 'N',
            Direction::South => 'S',
            Direction::West => 'W',
            Direction::East => 'E',
        }
    }

    pub fn forward_from(&self, pos: &Pos) -> Pos {
        self.forward_n_from(pos, 1)
    }

    pub fn forward_n_from(&self, pos: &Pos, n: isize) -> Pos {
        match self {
            Direction::North => (pos.0, pos.1 - n),
            Direction::South => (pos.0, pos.1 + n),
            Direction::West => (pos.0 - n, pos.1),
            Direction::East => (pos.0 + n, pos.1),
        }
    }

    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    pub fn invert(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

impl ops::Add<&Pos> for Direction {
    type Output = Pos;

    fn add(self, pos: &Pos) -> Self::Output {
        self.forward_from(pos)
    }
}

impl ops::Sub<&Pos> for Direction {
    type Output = Pos;

    fn sub(self, pos: &Pos) -> Self::Output {
        self.invert().forward_from(pos)
    }
}
