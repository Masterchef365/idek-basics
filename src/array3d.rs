#[derive(Clone)]
pub struct Array3D<T> {
    width: usize,
    height: usize,
    length: usize,
    data: Vec<T>,
}

pub type Index3D = (usize, usize, usize);

impl<T> Array3D<T> {
    pub fn from_array(width: usize, height: usize, data: Vec<T>) -> Self {
        assert_eq!(data.len() % (width * height), 0);
        assert_eq!(data.len() % width, 0);
        let length = data.len() / (width * height);

        Self { width, height, length, data }
    }

    pub fn new(width: usize, height: usize, length: usize) -> Self
    where
        T: Default + Copy,
    {
        Self {
            width,
            height,
            length,
            data: vec![T::default(); width * height * length],
        }
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    #[track_caller]
    fn calc_index(&self, (x, y, z): Index3D) -> usize {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        debug_assert!(z < self.length);
        x + (y * self.width) + z * (self.width * self.height)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

impl<T> std::ops::Index<Index3D> for Array3D<T> {
    type Output = T;
    fn index(&self, pos: Index3D) -> &T {
        &self.data[self.calc_index(pos)]
    }
}

impl<T> std::ops::IndexMut<Index3D> for Array3D<T> {
    fn index_mut(&mut self, pos: Index3D) -> &mut T {
        let idx = self.calc_index(pos);
        &mut self.data[idx]
    }
}
