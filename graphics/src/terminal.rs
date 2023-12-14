pub mod color;

use crate::render::texture::Texture;

pub trait Terminal {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn present(&mut self, tex: &Texture);
}
