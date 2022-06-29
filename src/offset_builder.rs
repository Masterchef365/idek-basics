use idek::{nalgebra::{Isometry3, Vector3}, Vertex};

#[derive(Default, Clone, Debug)]
pub struct GraphicsBuilder {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub transforms: Vec<Isometry3<f32>>,
}

impl GraphicsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Prepend a transformation
    pub fn push_tf(&mut self, tf: Isometry3<f32>) {
        let base = self.get_tf();
        self.transforms.push(base * tf);
    }

    /// Move up the transformation stack one step
    pub fn pop_tf(&mut self) -> Option<Isometry3<f32>> {
        self.transforms.pop()
    }

    /// Get current transformation
    pub fn get_tf(&mut self) -> Isometry3<f32> {
        self.transforms
            .last()
            .copied()
            .unwrap_or_else(|| Isometry3::identity())
    }

    /// Push a Vertex and return it's index
    pub fn push_vertex(&mut self, mut v: Vertex) -> u32 {
        let idx: u32 = self
            .vertices
            .len()
            .try_into()
            .expect("Vertex limit exceeded");

        let pos = self.get_tf() * Vector3::from(v.pos);
        v.pos = pos.into();

        self.vertices.push(v);
        idx
    }

    /// Push an index
    pub fn push_indices(&mut self, idx: &[u32]) {
        self.indices.extend_from_slice(idx);
    }

    /// Erase all content
    pub fn clear(&mut self) {
        self.indices.clear();
        self.vertices.clear();
    }
}
