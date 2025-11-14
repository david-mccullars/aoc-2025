use crate::{Pos, pos_from};
use aoc_parse::{Parser, parser, prelude::char_of, prelude::lines};
use std::collections::{HashMap, HashSet};

/// Provides a parser compatible with the aoc-parser to handle the standard
/// grid of characters, for example:
///
/// ```
/// ###############
/// #.......#....E#
/// #.#.###.#.###.#
/// #.....#.#...#.#
/// #.###.#####.#.#
/// #.#.#.......#.#
/// #.#.#####.###.#
/// #...........#.#
/// ###.#.#####.#.#
/// #...#.....#.#.#
/// #.#.#.###.#.#.#
/// #.....#...#.#.#
/// #.###.#.#.#.#.#
/// #S..#.....#...#
/// ###############
/// ```
///
/// Here there is always one character (typically '.') that represents an empty
/// space, and then there are multiple other characters that represents things
/// like walls, obstacles, robots, and more. The empty character MUST be the
/// the first character specified as an argument to the grid parser.
///
/// ```
/// let mut grid = parser!(grid(".#SE")).parse(input).unwrap();
/// let bounds = grid.bounds;               // => Pos
/// let walls = grid.take_all('#');         // => HashSet<Pos>
/// let start = grid.take_one('S');         // => Pos
/// let end = grid.take_one('E');           // => Pos
/// assert!(grid.take_all('.').is_empty())  // ignored
/// ```
#[derive(Debug, Default)]
pub struct Grid {
    pub map: HashMap<char, HashSet<Pos>>,
    pub bounds: Pos,
}

impl Grid {
    pub fn update(&mut self, c: char, pos: Pos) {
        self.map.entry(c).or_default().insert(pos);
    }

    pub fn update_bounds(&mut self, pos: &Pos) {
        if self.bounds.0 < pos.0 {
            self.bounds.0 = pos.0;
        }
        if self.bounds.1 < pos.1 {
            self.bounds.1 = pos.1;
        }
    }

    pub fn take_all(&mut self, c: char) -> HashSet<Pos> {
        self.map.remove(&c).unwrap_or_default()
    }

    pub fn take_one(&mut self, c: char) -> Pos {
        if let Some(s) = self.map.remove(&c) {
            if s.len() == 1 {
                s.into_iter().next().unwrap()
            } else {
                panic!(
                    "Grid contains too many elements for char {} ({})",
                    c,
                    s.len()
                )
            }
        } else {
            panic!("Grid is missing elements for char {}", c)
        }
    }
}

pub fn std_grid<P>(keep: P) -> impl Parser<Output = Grid>
where
    P: Parser<Output = char>,
{
    grid(keep, char_of("."))
}

pub fn grid<P, Q>(keep: P, ignored: Q) -> impl Parser<Output = Grid>
where
    P: Parser<Output = char>,
    Q: Parser,
{
    parser!(g:lines(({ k:keep => Some(k), ignored => None })+) => {
        let mut grid = Grid::default();
        for (y, row) in g.into_iter().enumerate() {
            for (x, c) in row.into_iter().enumerate() {
                let pos = pos_from(x, y);
                grid.update_bounds(&pos);
                if c.is_some() {
                    grid.update(c.unwrap(), pos);
                }
            }
        }
        grid
    })
}

pub fn grid_of(chars: &'static str) -> impl Parser<Output = Grid, RawOutput = (Grid,)> {
    parser!(g:lines(char_of(chars)+) => {
        let c: Vec<char> = chars.chars().collect();
        let mut grid = Grid::default();
        for (y, row) in g.into_iter().enumerate() {
            for (x, i) in row.into_iter().enumerate() {
                let pos = pos_from(x, y);
                grid.update_bounds(&pos);
                // First character in chars is ignored
                if i > 0 {
                    grid.update(c[i], pos);
                }
            }
        }
        grid
    })
}
