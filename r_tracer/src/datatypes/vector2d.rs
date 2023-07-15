use std::ops::Mul;
use std::ops::AddAssign;
use std::ops::Add;
use std::ops::MulAssign;
use std::cmp::PartialEq;
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
        let size: usize = width * height;
        let data: Vec<T> = vec![default; size];
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

    pub fn get(&self, row: usize, col: usize) -> &T {
        let index: usize = self.get_index(row, col);
        &self.data[index]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> Option<()> {
        let index: usize = self.get_index(row, col);
        self.data.get_mut(index).map(|element: &mut T| *element = value)
    }
}


impl<Color> MulAssign<f32> for Vector2D<Color>
where Color: MulAssign<f32>
{
    fn mul_assign(&mut self, other: f32) {
        for i in 0..self.data.len() {
            self.data[i] *= other;
        }
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

impl<T> Add<&Vector2D<T>> for &Vector2D<T>
where
    T: Add<T, Output = T> + Copy,
{
    type Output = Vector2D<T>;

    fn add(self, other: &Vector2D<T>) -> Vector2D<T> {
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

impl<T: PartialEq> PartialEq for Vector2D<T> {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.height == other.height
            && self.data == other.data
            && self.default == other.default
    }
}