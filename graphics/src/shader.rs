pub mod tex_mat;

use crate::raster::vertex::VertexTrait;
use glam::Vec4;

pub trait VertexProgram<T, TVertex>
where
    TVertex: VertexTrait,
{
    fn main(&self, uniforms: &T, vertex: &TVertex, varying: &mut TVertex, output: &mut Vec4);
}

pub trait FragmentProgram<T, TVertex>
where
    TVertex: VertexTrait,
{
    fn main(&self, uniforms: &T, varying: &TVertex, output: &mut Vec4);
}
