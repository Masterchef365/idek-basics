use idek::{nalgebra::{Similarity3, Point3}, Vertex};

#[derive(Default, Clone, Debug)]
pub struct OffsetBuilder {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub transforms: Vec<Similarity3<f32>>,
}

impl OffsetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Prepend a transformation
    pub fn push_tf(&mut self, tf: Similarity3<f32>) {
        let base = self.get_tf();
        self.transforms.push(base * tf);
    }

    /// Move up the transformation stack one step
    pub fn pop_tf(&mut self) -> Option<Similarity3<f32>> {
        self.transforms.pop()
    }

    /// Get current transformation
    pub fn get_tf(&mut self) -> Similarity3<f32> {
        self.transforms
            .last()
            .copied()
            .unwrap_or_else(|| Similarity3::identity())
    }

    /// Push a Vertex and return it's index
    pub fn push_vertex(&mut self, mut v: Vertex) -> u32 {
        let idx: u32 = self
            .vertices
            .len()
            .try_into()
            .expect("Vertex limit exceeded");

        let pos = self.get_tf() * Point3::from(v.pos);
        v.pos = pos.into();

        self.vertices.push(v);
        idx
    }

    /// Push an index
    pub fn push_indices(&mut self, idx: &[u32]) {
        self.indices.extend_from_slice(idx);
    }

    /// Append another graphics builder onto this one, transforming the shapes within
    pub fn append(&mut self, other: &Self) {
        let base = self.vertices.len() as u32;
        let tf = self.get_tf();
        self.vertices.extend(other.vertices.iter().copied().map(|mut v| {
            v.pos = (tf * Point3::from(v.pos)).into();
            v
        }));
        self.indices.extend(other.indices.iter().map(|i| i + base));
    }

    /// Erase all content
    pub fn clear(&mut self) {
        self.indices.clear();
        self.vertices.clear();
    }
}
