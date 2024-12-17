use std::ops::{Add, Mul, Sub};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {

    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn manhattan_distance(&self, other: &Point) -> i32 {
        i32::abs(other.x - self.x) + i32::abs(other.y - self.y)
    }
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Self { x, y }
    }
}

impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Self { x, y }
    }
}

impl Mul<i32> for Point {
    type Output = Point;
    fn mul(self, rhs: i32) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        Self { x, y }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct Point64 {
    pub x: i64,
    pub y: i64,
}

impl Point64 {

    #[must_use]
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn from_point(point: Point) -> Self {
        Self { x: i64::from(point.x), y: i64::from(point.y) }
    }

    #[must_use]
    pub fn to_f64(&self) -> (f64, f64) {
        (self.x as f64, self.y as f64)
    }
}

impl Add<Point64> for Point64 {
    type Output = Point64;
    fn add(self, rhs: Point64) -> Point64 {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Self { x, y }
    }
}

impl Add<i64> for Point64 {
    type Output = Point64;
    fn add(self, rhs: i64) -> Self::Output {
        let x = self.x + rhs;
        let y = self.y + rhs;
        Self { x, y }
    }
}

impl Sub<Point64> for Point64 {
    type Output = Point64;
    fn sub(self, rhs: Point64) -> Point64 {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Self { x, y }
    }
}

impl Mul<i64> for Point64 {
    type Output = Point64;
    fn mul(self, rhs: i64) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        Self { x, y }
    }
}
