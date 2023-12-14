use glam::Vec4;

pub trait RenderTarget {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn set(&mut self, x: usize, y: usize, color: &Vec4);
    fn clear(&mut self, color: Vec4);
}
