pub mod tex_mat;

use crate::raster::vertex::Vertex;
use glam::Vec4;

pub trait VertexProgram<T> {
    fn main(&self, uniforms: &T, vertex: &Vertex, varying: &mut Vertex, output: &mut Vec4);
}

pub trait FragmentProgram<T> {
    fn main(&self, uniforms: &T, varying: &Vertex, output: &mut Vec4);
}
