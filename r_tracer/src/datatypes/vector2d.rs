pub struct Vector2D<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
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