extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;
use std::ops::Range;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<isize> {
    Polygon::parse(input)
        .possible_rectangles()
        .map(|rect| rect.area())
        .max()
}

pub fn part_two(input: &str) -> Option<isize> {
    Polygon::parse(input)
        .valid_rectangles()
        .map(|rect| rect.area())
        .max()
}

struct Polygon {
    vertices: Vec<Pos>,
    edges: Vec<Edge>,
}

impl Polygon {
    fn parse(input: &str) -> Self {
        let vertices = parser!(lines(isize "," isize))
            .parse(input)
            .expect("Failed to parse");

        let edges: Vec<Edge> = vertices
            .iter()
            .chain(std::iter::once(&vertices[0]))
            .tuple_windows()
            .map(|(a, b)| Edge::new(a, b))
            .collect();

        Self { vertices, edges }
    }

    fn possible_rectangles(&self) -> impl Iterator<Item = Rect> {
        self.vertices.iter().combinations(2).map(|c| Rect::new(c))
    }

    fn valid_rectangles(&self) -> impl Iterator<Item = Rect> {
        self.possible_rectangles()
            .into_iter()
            .filter(move |rect| !rect.is_cut_by_any(&self.edges))
            .filter(move |rect| rect.are_corners_in(&self))
    }

    fn vertical_edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.iter().filter(|e| matches!(e, Edge::V { .. }))
    }

    fn is_on_edge(&self, pos: &Pos) -> bool {
        self.edges.iter().any(|e| e.contains(pos))
    }

    fn is_bounded_by_edges(&self, pos: &Pos) -> bool {
        let edge_crossings = self
            .vertical_edges()
            .filter(|e| e.crosses_horizontal_ray(pos))
            .count();
        edge_crossings % 2 == 1
    }

    fn contains(&self, pos: &Pos) -> bool {
        self.is_on_edge(pos) || self.is_bounded_by_edges(pos)
    }
}

#[derive(Clone, Copy)]
enum Edge {
    H { y: isize, x1: isize, x2: isize },
    V { x: isize, y1: isize, y2: isize },
}

impl Edge {
    fn new(a: &Pos, b: &Pos) -> Self {
        if a.0 == b.0 {
            Edge::V {
                x: a.0,
                y1: a.1.min(b.1),
                y2: a.1.max(b.1),
            }
        } else if a.1 == b.1 {
            Edge::H {
                y: a.1,
                x1: a.0.min(b.0),
                x2: a.0.max(b.0),
            }
        } else {
            panic!("Invalid edge")
        }
    }

    fn contains(&self, pos: &Pos) -> bool {
        match self {
            Edge::H { y, x1, x2 } => *y == pos.1 && (*x1..=*x2).contains(&pos.0),
            Edge::V { x, y1, y2 } => *x == pos.0 && (*y1..=*y2).contains(&pos.1),
        }
    }

    fn crosses_horizontal_ray(&self, pos: &Pos) -> bool {
        match self {
            Edge::H { .. } => panic!("Not vertical"),
            Edge::V { x, y1, y2 } => *x > pos.0 && (*y1..*y2).contains(&pos.1),
        }
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Rect {
    x1: isize,
    x2: isize,
    y1: isize,
    y2: isize,
}

impl Rect {
    fn new(vertices: Vec<&Pos>) -> Self {
        let a = vertices[0];
        let b = vertices[1];

        let (x1, x2) = (a.0.min(b.0), a.0.max(b.0));
        let (y1, y2) = (a.1.min(b.1), a.1.max(b.1));

        Self { x1, x2, y1, y2 }
    }

    fn are_corners_in(&self, polygon: &Polygon) -> bool {
        [
            (self.x1, self.y1),
            (self.x2, self.y2),
            (self.x1, self.y2),
            (self.x2, self.y1),
        ]
        .into_iter()
        .all(|corner| polygon.contains(&corner))
    }

    fn area(&self) -> isize {
        (self.x2 - self.x1 + 1) * (self.y2 - self.y1 + 1)
    }

    fn x_interior(&self) -> Range<isize> {
        (self.x1 + 1)..self.x2
    }

    fn y_interior(&self) -> Range<isize> {
        (self.y1 + 1)..self.y2
    }

    fn x_cut_by(&self, range: Range<isize>) -> bool {
        range.contains(&self.x1) || range.contains(&self.x2)
    }

    fn y_cut_by(&self, range: Range<isize>) -> bool {
        range.contains(&self.y1) || range.contains(&self.y2)
    }

    fn is_cut_by_any(&self, edges: &[Edge]) -> bool {
        edges.iter().any(|edge| match edge {
            Edge::H { y, x1, x2 } => self.y_interior().contains(y) && self.x_cut_by(*x1..*x2),
            Edge::V { x, y1, y2 } => self.x_interior().contains(x) && self.y_cut_by(*y1..*y2),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
