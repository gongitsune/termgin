use super::{FragmentProgram, VertexProgram};
use crate::{material::tex_mat::Uniform, raster::vertex::Vertex};
use glam::Vec4;

pub struct TexMatVertShader {}
pub struct TexMatFragShader {}

impl VertexProgram<Uniform> for TexMatVertShader {
    fn main(&self, uniforms: &Uniform, vertex: &Vertex, varying: &mut Vertex, output: &mut Vec4) {
        varying.pos = vertex.pos;
        varying.normal = vertex.normal;
        varying.uv = vertex.uv;

        let wvp = uniforms.world * uniforms.view * uniforms.projection;
        *output = wvp * vertex.pos;
    }
}

impl FragmentProgram<Uniform> for TexMatFragShader {
    fn main(&self, uniforms: &Uniform, varying: &Vertex, output: &mut Vec4) {
        let uv = varying.uv;
        uniforms.main_tex.sample_to_out(uv.x, uv.y, output);
    }
}
