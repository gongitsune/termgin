use crate::shader::{FragmentProgram, VertexProgram};
pub mod tex_mat;

pub trait Material<TUniform, TVert, TFrag>
where
    TVert: VertexProgram<TUniform>,
    TFrag: FragmentProgram<TUniform>,
{
    fn vert_shader(&self) -> &TVert;
    fn frag_shader(&self) -> &TFrag;
    fn uniform(&self) -> &TUniform;
}
