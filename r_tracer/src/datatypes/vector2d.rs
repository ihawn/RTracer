use std::ops::Mul;
use std::ops::AddAssign;
use std::ops::Add;
use crate::datatypes::color::Color;

#[derive(Clone)]
pub struct Vector2D<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
    pub default: T
}

impl<T> Vector2D<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self
    where
        T: Clone,
        T: Copy
    {
        let size = width * height;
        let data = vec![default; size];
        Self {
            width,
            height,
            data,
            default
        }
    }

    pub fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        let index = self.get_index(row, col);
        self.data.get(index)
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> Option<()> {
        let index = self.get_index(row, col);
        self.data.get_mut(index).map(|element| *element = value)
    }
}


impl<T> Mul<f64> for Vector2D<T>
where
    T: Mul<f64, Output = T> + Copy,
{
    type Output = Vector2D<T>;

    fn mul(self, scalar: f64) -> Vector2D<T> {
        let mut result = self.clone();
        for i in 0..self.data.len() {
            result.data[i] = self.data[i] * scalar;
        }
        result
    }
}

impl<T> Mul<Vector2D<Color>> for Vector2D<T>
where
    T: Mul<Color, Output = T> + AddAssign + Copy,
{
    type Output = Vector2D<T>;

    fn mul(self, other: Vector2D<Color>) -> Vector2D<T> {
        assert_eq!(self.width, other.width);
        assert_eq!(self.height, other.height);

        let mut result = Vector2D::new(self.width, self.height, self.default);

        for i in 0..self.data.len() {
            result.data[i] = self.data[i] * other.data[i];
        }

        result
    }
}

impl<T> Add<Vector2D<T>> for Vector2D<T>
where
    T: Add<T, Output = T> + Copy,
{
    type Output = Vector2D<T>;

    fn add(self, other: Vector2D<T>) -> Vector2D<T> {
        assert_eq!(self.width, other.width);
        assert_eq!(self.height, other.height);

        let mut result = Vector2D::new(self.width, self.height, self.default);

        for i in 0..self.data.len() {
            result.data[i] = self.data[i] + other.data[i];
        }

        result
    }
}

impl<T> AddAssign<Vector2D<T>> for Vector2D<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, other: Vector2D<T>) {
        assert_eq!(self.width, other.width);
        assert_eq!(self.height, other.height);

        for i in 0..self.data.len() {
            self.data[i] += other.data[i];
        }
    }
}