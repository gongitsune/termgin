pub struct DepthBuffer {
    width: usize,
    height: usize,
    data: Vec<f32>,
}

impl DepthBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0.; width * height],
        }
    }

    pub fn clear(&mut self) {
        for d in &mut self.data {
            *d = f32::MIN_POSITIVE;
        }
    }

    pub fn set(&mut self, x: usize, y: usize, depth: f32) {
        let index = y * self.width + x;
        self.data[index] = depth;
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        let index = y * self.width + x;
        self.data[index]
    }
}
