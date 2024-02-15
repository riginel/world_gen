use std::ops;

/// Struct representing a 2-dimensional point
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Vector2 {
    pub x: usize,
    pub y: usize,
}

impl Vector2 {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn manhattan_distance(&self, other: Self) -> u32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
    }

    pub fn neighbours(&self, rows: usize, columns: usize) -> Vec<Self> {
        let mut vec: Vec<Self> = Vec::with_capacity(4);
        //⇐
        if self.x > 0 {
            vec.push(Self::new(self.x - 1, self.y))
        }
        //⇒
        if self.x < rows - 1 {
            vec.push(Self::new(self.x + 1, self.y))
        }
        //⇑
        if self.y > 0 {
            vec.push(Self::new(self.x, self.y - 1))
        }
        //⇓
        if self.y < columns - 1 {
            vec.push(Self::new(self.x, self.y + 1))
        }
        vec
    }
}

impl ops::Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}
