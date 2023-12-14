use crate::raster::target::RenderTarget;
use glam::Vec4;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    data: Vec<f32>,
}

impl Texture {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0.0; width * height * 4],
        }
    }

    fn offset(&self, x: usize, y: usize) -> usize {
        let mx = x % self.width;
        let my = y % self.height;
        (mx + my * self.width) * 4
    }

    pub fn get_to_out(&self, x: usize, y: usize, output: &mut Vec4) {
        let offset = self.offset(x, y);
        output.x = self.data[offset + 0];
        output.y = self.data[offset + 1];
        output.z = self.data[offset + 2];
        output.w = self.data[offset + 3];
    }

    pub fn sample_to_out(&self, x: f32, y: f32, output: &mut Vec4) {
        let px = (x.abs() * self.width as f32) as usize % self.width;
        let py = (y.abs() * self.height as f32) as usize % self.height;
        let offset = self.offset(px, py);
        output.x = self.data[offset + 0];
        output.y = self.data[offset + 1];
        output.z = self.data[offset + 2];
        output.w = self.data[offset + 3];
    }

    pub fn clear(&mut self, color: &Vec4) {
        for i in (0..self.data.len()).step_by(4) {
            color.write_to_slice(&mut self.data[i..i + 4]);
        }
    }
}

impl RenderTarget for Texture {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn set(&mut self, x: usize, y: usize, color: &Vec4) {
        let offset = self.offset(x, y);
        let slice = self.data[offset..offset + 4].as_mut();
        color.write_to_slice(slice);
    }

    fn clear(&mut self, color: Vec4) {
        for i in (0..self.data.len()).step_by(4) {
            color.write_to_slice(&mut self.data[i..i + 4]);
        }
    }
}
