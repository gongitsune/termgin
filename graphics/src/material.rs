use crate::{
    raster::vertex::VertexTrait,
    shader::{FragmentProgram, VertexProgram},
};
pub mod tex_mat;

pub trait Material<TUniform, TVert, TFrag, TVertex>
where
    TVert: VertexProgram<TUniform, TVertex>,
    TFrag: FragmentProgram<TUniform, TVertex>,
    TVertex: VertexTrait,
{
    fn vert_shader(&self) -> &TVert;
    fn frag_shader(&self) -> &TFrag;
}
