use super::{FragmentProgram, VertexProgram};
use crate::{material::tex_mat::Uniform, raster::vertex::SimpleVertex};
use glam::Vec4;

pub struct TexMatVertShader {}
pub struct TexMatFragShader {}

impl VertexProgram<Uniform, SimpleVertex> for TexMatVertShader {
    fn main(
        &self,
        uniforms: &Uniform,
        vertex: &SimpleVertex,
        varying: &mut SimpleVertex,
        output: &mut Vec4,
    ) {
        varying.pos = vertex.pos;
        varying.normal = vertex.normal;
        varying.uv = vertex.uv;

        let wv = uniforms.view * uniforms.world;
        *output = uniforms.projection * wv * vertex.pos;
    }
}

impl FragmentProgram<Uniform, SimpleVertex> for TexMatFragShader {
    fn main(&self, uniforms: &Uniform, varying: &SimpleVertex, output: &mut Vec4) {
        let uv = varying.uv;
        uniforms.main_tex.sample_to_out(uv.x, uv.y, output);
    }
}
