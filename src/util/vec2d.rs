use std::ops::{Add, Index, Sub};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {

    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Point { x, y}
    }
}

impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Point { x, y}
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum Directions {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

#[derive(Clone)]
pub struct Vec2d<T> 
    where T: Clone
{
    pub grid: Vec<T>,
    pub line_len: i32,
}

impl<T> Vec2d<T> 
    where T: Clone
{
    #[must_use]
    pub fn in_bounds(&self, point: Point) -> bool {
        let max_y = self.grid.len() as i32 / self.line_len;
        point.x >= 0 && point.y >= 0 && point.x < self.line_len && point.y < max_y 
    }

    /// # Panics
    /// If you give an index out of bounds of a signed 32 bit integer
    #[must_use]
    pub fn idx_to_point(&self, idx: usize) -> Point {
        let idx: i32 = idx.try_into().expect("Invalid index");
        Point::new(idx % self.line_len, idx / self.line_len)
    }

    /// # Panics
    /// if you pass a point that cannot be converted to an index.
    /// For example: a point with a negative value for x of y.
    /// Validate your point first using the [`in_bounds`] method
    #[must_use]
    pub fn point_to_idx(&self, point: Point) -> usize {
        (point.y * self.line_len + point.x)
            .try_into()
            .expect("Invalid Point -> index")
    }

    #[must_use]
    pub fn next_point(&self, point: Point, direction: Directions) -> Option<Point> {
        let next = match direction {
            Directions::Down => Point::new(point.x, point.y + 1),
            Directions::DownLeft => Point::new(point.x - 1, point.y + 1),
            Directions::DownRight => Point::new(point.x + 1, point.y + 1),
            Directions::Up => Point::new(point.x, point.y - 1),
            Directions::UpLeft => Point::new(point.x - 1, point.y - 1),
            Directions::UpRight => Point::new(point.x + 1, point.y - 1),
            Directions::Left => Point::new(point.x - 1, point.y),
            Directions::Right => Point::new(point.x + 1, point.y),
        };
        if self.in_bounds(next) {
            Some(next)
        } else {
            None
        }
    }
}

impl <T: Clone> Index<Point> for Vec2d<T>{
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        let idx = self.point_to_idx(index);
        &self.grid[idx]
    }
}