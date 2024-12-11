use std::{
    fmt::{self, Debug, Display},
    ops::{Add, Index},
};

use rand::{Rng, distributions::Standard, prelude::Distribution};

type Coord = i32;

pub const NORTH: Point = Point::new(0, -1);
pub const NORTH_EAST: Point = Point::new(1, -1);
pub const EAST: Point = Point::new(1, 0);
pub const SOUTH_EAST: Point = Point::new(1, 1);
pub const SOUTH: Point = Point::new(0, 1);
pub const SOUTH_WEST: Point = Point::new(-1, 1);
pub const WEST: Point = Point::new(-1, 0);
pub const NORTH_WEST: Point = Point::new(-1, -1);

pub const ORTHO_DIR: [Point; 4] = [NORTH, SOUTH, WEST, EAST];
pub const ORTHO_PLUS_DIR: [Point; 8] = [
    NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST, NORTH_WEST,
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: Coord,
    pub y: Coord,
}
impl AsRef<Point> for Point {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Point {
    #[inline]
    #[must_use]
    pub const fn new(x: Coord, y: Coord) -> Self {
        Point { x, y }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CellState {
    Alive,
    Dead,
}
impl Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellState::Dead => {
                write!(f, "💀")?;
            }
            CellState::Alive => {
                write!(f, "😇")?;
            }
        }
        Ok(())
    }
}

impl Distribution<CellState> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CellState {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=1) {
            // rand 0.8
            0 => CellState::Alive,
            1 => CellState::Dead,
            _ => CellState::Alive,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct NeighbourState {
    dead: i32,
    alive: i32,
}
#[derive(Debug)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<T>,
}

impl<T> Grid<T> {
    pub fn contains(&self, p: &Point) -> bool {
        p.x >= 0 && (p.x as usize) < self.width && p.y >= 0 && (p.y as usize) < self.height
    }

    fn pos(&self, p: usize) -> Point {
        Point::new((p % self.width) as i32, (p / self.width) as i32)
    }
    fn idx(&self, p: &Point) -> usize {
        ((self.width as i32) * p.y + p.x) as usize
    }

    pub fn try_get<U: AsRef<Point>>(&self, p: U) -> Option<&T> {
        if self.contains(p.as_ref()) {
            Some(&self[*p.as_ref()])
        } else {
            None
        }
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, pos: Point) -> &Self::Output {
        &self.cells[self.idx(&pos)]
    }
}

impl Grid<CellState> {
    pub fn new_random(width: usize, height: usize) -> Self {
        let size = width * height;
        let cells: Vec<CellState> = (0..size).map(|_| rand::random()).collect();
        Grid {
            width,
            height,
            cells,
        }
    }
    /*
    Any live cell with 0 or 1 live neighbors becomes dead, because of underpopulation
    Any live cell with 2 or 3 live neighbors stays alive, because its neighborhood is just right
    Any live cell with more than 3 live neighbors becomes dead, because of overpopulation
    Any dead cell with exactly 3 live neighbors becomes alive, by reproduction
     */
    pub fn update_states(&mut self) {
        let mut new_grid: Vec<CellState> = Vec::new();
        for (idx, &cell) in self.cells.iter().enumerate() {
            let state = self.get_neighbours_state(self.pos(idx));
            match (&cell, state.alive) {
                (CellState::Alive, 0..=1) => new_grid.push(CellState::Dead),
                (CellState::Alive, 2..=3) => new_grid.push(CellState::Alive),
                (CellState::Alive, 4..=8) => new_grid.push(CellState::Dead),
                (CellState::Dead, 3) => new_grid.push(CellState::Alive),
                (_, _) => new_grid.push(cell),
            }
        }
        self.cells = new_grid;
    }
    fn get_neighbours_state(&self, point: Point) -> NeighbourState {
        let mut alive = 0;
        let mut dead = 0;
        for neighbour in self.get_neighbours(point).map(|p| self.try_get(p)) {
            match neighbour {
                Some(c) => match c {
                    CellState::Alive => alive += 1,
                    CellState::Dead => dead += 1,
                },
                None => {
                    continue;
                }
            }
        }
        NeighbourState { alive, dead }
    }

    fn get_neighbours(&self, point: Point) -> impl Iterator<Item = Point> {
        ORTHO_PLUS_DIR
            .into_iter()
            .map(move |d| point + d)
            .filter(|p| self.contains(p))
    }

    // fn to_string() -> String {

    // }
}

impl Default for Grid<CellState> {
    fn default() -> Self {
        Grid::new_random(10, 10)
    }
}

impl Display for Grid<CellState> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for w in row * self.width..(row + 1) * self.width {
                write!(f, "{}", self.cells[w])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Debug> Grid<T> {
    pub fn print(&self) {
        println!("Grid {w}x{h}", w = &self.width, h = &self.height);
        for row in 0..self.height {
            println!(
                "r{row}: {:?}",
                &self.cells[row * self.width..(row + 1) * self.width]
            );
        }
    }
}
