#![warn(missing_docs)]
//! Game of Life
//!
//! Library to manage the grid state for Conways game of life.
//!
//! See: <https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life>
//!
//! ```
//! use gridlife::Grid;
//! let mut grid = Grid::new_random(3, 3);
//! let mut population = grid.population;
//! // Run the rules of Game of Life until the population count stabalizes
//! loop {
//!     grid.update_states();
//!     let next_pop = grid.population;
//!     if next_pop == population {
//!         break;
//!     }
//!     population = next_pop;
//! }
//! ```
use std::{
    fmt::{self, Debug, Display},
    ops::{Add, Index},
};

type Coord = i32;

const NORTH: Point = Point::new(0, -1);
const NORTH_EAST: Point = Point::new(1, -1);
const EAST: Point = Point::new(1, 0);
const SOUTH_EAST: Point = Point::new(1, 1);
const SOUTH: Point = Point::new(0, 1);
const SOUTH_WEST: Point = Point::new(-1, 1);
const WEST: Point = Point::new(-1, 0);
const NORTH_WEST: Point = Point::new(-1, -1);

const ORTHO_PLUS_DIR: [Point; 8] = [
    NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST, NORTH_WEST,
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point {
    x: Coord,
    y: Coord,
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
/// `CellState` models whether a cell has an alive or dead population
pub enum CellState {
    /// `Alive` with a `char` to be rendered
    Alive(char),
    /// `Dead` with a `char` to be rendered
    Dead(char),
}
impl Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellState::Dead(c) => {
                write!(f, "{c}")?;
            }
            CellState::Alive(c) => {
                write!(f, "{c}")?;
            }
        }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct NeighbourState {
    dead: i32,
    alive: i32,
}
#[derive(Debug)]
#[non_exhaustive]
/// `Grid` holds the state for a Conways game of life
/// ```
/// use gridlife::Grid;
/// let grid = Grid {
///     width: 3,
///     height: 3,
///     cells: vec!['1', '1', '1', '0', '1', '1', '0', '0', '1'],
///     dead_glyph: '1',
///     alive_glyph: '0',
///     population: 0
/// };
/// ```
pub struct Grid<T> {
    /// The `width` of the grid to be created
    pub width: usize,
    /// The `height` of the grid to be created
    pub height: usize,
    /// The state of the grid in terms of what cells are alive and dead in automaton
    pub cells: Vec<T>,
    /// What character glyph should be used to display a dead population
    pub dead_glyph: char,
    /// What character glyph should be used to display an alive population
    pub alive_glyph: char,
    /// Population of the grid i.e number of alive cells
    pub population: usize,
}

impl<T> Grid<T> {
    fn contains(&self, p: &Point) -> bool {
        p.x >= 0 && (p.x as usize) < self.width && p.y >= 0 && (p.y as usize) < self.height
    }

    fn pos(&self, p: usize) -> Point {
        Point::new((p % self.width) as i32, (p / self.width) as i32)
    }
    fn idx(&self, p: &Point) -> usize {
        ((self.width as i32) * p.y + p.x) as usize
    }

    fn try_get<U: AsRef<Point>>(&self, p: U) -> Option<&T> {
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
    /// Create a new `Grid` of a given `width` and `height`.
    /// It will default to `X` for alive glyph and ` ` for dead glyph
    /// ```
    /// use gridlife::Grid;
    /// let grid = Grid::new_empty(3, 3);
    /// ```
    pub fn new_empty(width: usize, height: usize) -> Self {
        let size = width * height;
        let cells: Vec<CellState> = (0..size).map(|_| CellState::Dead(' ')).collect();
        Grid {
            width,
            height,
            cells,
            ..Default::default()
        }
    }

    fn generate_random_cells(size: usize, alive_glyph: char, dead_glyph: char) -> Vec<CellState> {
        (0..size)
            .map(|_| {
                if fastrand::bool() {
                    CellState::Alive(alive_glyph)
                } else {
                    CellState::Dead(dead_glyph)
                }
            })
            .collect()
    }
    /// Generate a new `Grid` of a given `width` and `height`
    /// It will be populated with a random distribution of Alive/Dead cells
    /// The default glyphs of `X` for alive and ` ` for dead.
    /// ```
    /// use gridlife::Grid;
    /// let grid = Grid::new_random(3, 3);
    /// ```
    pub fn new_random(width: usize, height: usize) -> Self {
        let default = Self::default();
        let cells: Vec<CellState> =
            Self::generate_random_cells(width * height, default.alive_glyph, default.dead_glyph);
        Grid {
            width,
            height,
            cells,
            ..default
        }
    }

    /// Generate a new `Grid` of a given `width` and `height`
    /// It will be populated with a random distribution of Alive/Dead cells
    /// The glyphs can be overriddne with `alive_glyph` and `dead_glyph`
    /// ```
    /// use gridlife::Grid;
    /// let grid = Grid::new_random_custom_glyphs(3, 3, '1', '0');
    /// ```
    pub fn new_random_custom_glyphs(
        width: usize,
        height: usize,
        alive_glyph: char,
        dead_glyph: char,
    ) -> Self {
        let cells = Self::generate_random_cells(width * height, alive_glyph, dead_glyph);
        let population = cells
            .iter()
            .filter(|&&c| c == CellState::Alive(alive_glyph))
            .count();
        Grid {
            width,
            height,
            cells,
            alive_glyph,
            dead_glyph,
            population,
        }
    }
    /// Re-generates the state of the `Grid` `cells` based on the rules of Conways game of life
    pub fn update_states(&mut self) -> &[CellState] {
        let mut new_grid: Vec<CellState> = Vec::new();
        for (idx, &cell) in self.cells.iter().enumerate() {
            let state = self.get_neighbours_state(self.pos(idx));
            let cellstate = self.get_cell_state(&cell, state);
            new_grid.push(cellstate);
        }
        self.cells = new_grid;
        self.population = self.calculate_population();
        &self.cells
    }
    fn calculate_population(&self) -> usize {
        self.cells
            .iter()
            .filter(|&&c| c == CellState::Alive(self.alive_glyph))
            .count()
    }
    /// Gets the new state of the current cell based on the following rules:
    /// - Any live cell with 0 or 1 live neighbors becomes dead, because of underpopulation
    /// - Any live cell with 2 or 3 live neighbors stays alive, because its neighborhood is just right
    /// - Any live cell with more than 3 live neighbors becomes dead, because of overpopulation
    /// - Any dead cell with exactly 3 live neighbors becomes alive, by reproduction
    fn get_cell_state(&self, cell: &CellState, state: NeighbourState) -> CellState {
        match (&cell, state.alive) {
            (CellState::Alive(_), 0..=1) => CellState::Dead(self.dead_glyph),
            (CellState::Alive(_), 2..=3) => CellState::Alive(self.alive_glyph),
            (CellState::Alive(_), 4..=8) => CellState::Dead(self.dead_glyph),
            (CellState::Dead(_), 3) => CellState::Alive(self.alive_glyph),
            (_, _) => *cell,
        }
    }
    fn get_neighbours_state(&self, point: Point) -> NeighbourState {
        let mut alive = 0;
        let mut dead = 0;
        for neighbour in self.get_neighbours(point).map(|p| self.try_get(p)) {
            match neighbour {
                Some(c) => match c {
                    CellState::Alive(_) => alive += 1,
                    CellState::Dead(_) => dead += 1,
                },
                None => {
                    continue;
                }
            }
        }
        NeighbourState { alive, dead }
    }

    fn get_neighbours(&self, point: Point) -> impl Iterator<Item = Point> + use<'_> {
        ORTHO_PLUS_DIR
            .into_iter()
            .map(move |d| point + d)
            .filter(|p| self.contains(p))
    }
}

impl Default for Grid<CellState> {
    fn default() -> Self {
        let size = 10 * 10;
        let cells: Vec<CellState> = (0..size).map(|_| CellState::Dead(' ')).collect();
        Grid {
            width: 10,
            height: 10,
            cells,
            alive_glyph: 'X',
            dead_glyph: ' ',
            population: 0,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_try_get() {
        let g = Grid::new_empty(0, 0);
        assert!(g.try_get(Point { x: 10, y: 10 }) == None);
    }

    #[test]
    fn test_grid_new_random() {
        let rand_g = Grid::new_random(10, 10);
        assert_eq!(rand_g.cells.len(), 100);
    }

    #[test]
    fn test_get_neighbours_state() {
        let mut g = Grid::new_empty(3, 3);
        g.cells[1] = CellState::Alive(g.alive_glyph);
        // x 0 x
        // x x x
        // x x x
        let state = g.get_neighbours_state(Point { x: 0, y: 0 });
        assert_eq!(state.dead, 2);
        assert_eq!(state.alive, 1);
    }

    #[test]
    fn test_get_neighbours_state_unknown_point() {
        let g = Grid::new_empty(3, 3);
        let state = g.get_neighbours_state(Point { x: 5, y: 5 });
        assert_eq!(state.dead, 0);
        assert_eq!(state.alive, 0);
    }

    #[test]
    fn test_grid_display() {
        let mut g = Grid::new_empty(3, 3);
        g.cells[4] = CellState::Alive('X');
        let s = format!("{}", g);
        assert_eq!(s, "   \n X \n   \n".to_string());
    }

    #[test]
    fn test_grid_debug() {
        let mut g = Grid::new_empty(3, 3);
        g.cells[4] = CellState::Alive('X');
        let s = format!("{:?}", g);
        assert_eq!(s, "Grid { width: 3, height: 3, cells: [Dead(' '), Dead(' '), Dead(' '), Dead(' '), Alive('X'), Dead(' '), Dead(' '), Dead(' '), Dead(' ')], dead_glyph: ' ', alive_glyph: 'X', population: 0 }".to_string());
    }

    #[test]
    fn test_update_state() {
        let mut g = Grid::new_random(10, 10);
        g.update_states();
    }

    #[test]
    fn test_get_cell_state() {
        let g = Grid::new_empty(3, 3);
        // Any live cell with 0 or 1 live neighbors becomes dead, because of underpopulation
        assert_eq!(
            g.get_cell_state(&CellState::Alive('X'), NeighbourState { alive: 1, dead: 0 }),
            CellState::Dead(' ')
        );
        //Any live cell with 2 or 3 live neighbors stays alive, because its neighborhood is just right
        assert_eq!(
            g.get_cell_state(&CellState::Alive('X'), NeighbourState { alive: 3, dead: 0 }),
            CellState::Alive('X')
        );
        // Any live cell with more than 3 live neighbors becomes dead, because of overpopulation
        assert_eq!(
            g.get_cell_state(&CellState::Alive('X'), NeighbourState { alive: 5, dead: 1 }),
            CellState::Dead(' ')
        );
        // Any dead cell with exactly 3 live neighbors becomes alive, by reproduction
        assert_eq!(
            g.get_cell_state(&CellState::Dead(' '), NeighbourState { alive: 3, dead: 0 }),
            CellState::Alive('X')
        );
    }

    #[test]
    fn test_new_random_custom_glyphs() {
        let g = Grid::new_random_custom_glyphs(3, 3, 'A', 'D');
        assert_eq!(g.cells.len(), 9);
        let unexpected_states: Vec<&CellState> = g
            .cells
            .iter()
            .filter(|c| {
                let state = c.to_string();
                state != "A" && state != "D"
            })
            .collect();
        assert_eq!(unexpected_states.len(), 0);
    }
}
