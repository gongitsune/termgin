use glam::Vec4;

use super::vertex::Vertex;

pub trait VertexProgram<T> {
    fn main(&self, uniforms: &T, vertex: &Vertex, varying: &mut Vertex, output: &mut Vec4);
}

pub trait FragmentProgram<T> {
    fn main(&self, uniforms: &T, varying: &Vertex, output: &mut Vec4);
}
