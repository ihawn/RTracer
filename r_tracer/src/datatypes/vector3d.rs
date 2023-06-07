#[derive(Clone)]
pub struct Vector3D<T> {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub data: Vec<T>,
    pub default: T,
}

impl<T> Vector3D<T> {
    pub fn new(width: usize, height: usize, depth: usize, default: T) -> Self
    where
        T: Clone + Copy,
    {
        let size = width * height * depth;
        let data = vec![default; size];
        Self {
            width,
            height,
            depth,
            data,
            default,
        }
    }

    pub fn get_index(&self, row: usize, col: usize, depth: usize) -> usize {
        row * self.width * self.depth + col * self.depth + depth
    }

    pub fn get(&self, row: usize, col: usize, depth: usize) -> Option<&T> {
        let index = self.get_index(row, col, depth);
        self.data.get(index)
    }

    pub fn set(&mut self, row: usize, col: usize, depth: usize, value: T) -> Option<()> {
        let index = self.get_index(row, col, depth);
        self.data.get_mut(index).map(|element| *element = value)
    }
}