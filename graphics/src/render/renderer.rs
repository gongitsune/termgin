use super::mesh::Mesh;
use crate::{
    raster::{depth::DepthBuffer, raster::triangle, target::RenderTarget},
    shader::{FragmentProgram, VertexProgram},
};
use glam::Vec4;

pub struct Renderer<TUniform> {
    pub uniform_buffer: TUniform,
}

impl<TUniform> Renderer<TUniform> {
    pub fn new(uniform_buffer: TUniform) -> Self {
        Self { uniform_buffer }
    }

    pub fn draw_mesh(
        &self,
        mesh: &Mesh,
        vert_shader: &impl VertexProgram<TUniform>,
        frag_shader: &impl FragmentProgram<TUniform>,
        depth: &mut DepthBuffer,
        target: &mut impl RenderTarget,
    ) {
        for i in (0..mesh.indices.len()).step_by(3) {
            let v0 = mesh.vertices[mesh.indices[i] as usize];
            let v1 = mesh.vertices[mesh.indices[i + 1] as usize];
            let v2 = mesh.vertices[mesh.indices[i + 2] as usize];
            triangle(
                vert_shader,
                frag_shader,
                depth,
                target,
                &self.uniform_buffer,
                &[v0, v1, v2],
            )
        }
    }

    pub fn clear(&self, target: &mut impl RenderTarget, depth: &mut DepthBuffer, color: Vec4) {
        target.clear(color);
        depth.clear();
    }
}
