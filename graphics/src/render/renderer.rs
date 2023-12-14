use std::ops::DerefMut;

use super::mesh::Mesh;
use crate::{
    raster::{depth::DepthBuffer, raster::triangle, target::RenderTarget},
    shader::{FragmentProgram, VertexProgram},
};
use glam::Mat4;

pub enum Command<'a, TUniform> {
    SetRenderTarget {
        target: &'a mut dyn RenderTarget,
    },
    SetDepthBuffer {
        depth_buffer: &'a mut DepthBuffer,
    },
    SetVertexShader {
        shader: &'a dyn VertexProgram<TUniform>,
    },
    SetFragmentShader {
        shader: &'a dyn FragmentProgram<TUniform>,
    },
    DrawMesh {
        mesh: Mesh,
        transform: Mat4,
    },
}

pub struct Renderer<'a, TUniform> {
    commands: Vec<Command<'a, TUniform>>,
    uniform_buffer: TUniform,
}

impl<'a, TUniform> Renderer<'a, TUniform> {
    pub fn new(uniform_buffer: TUniform) -> Self {
        Self {
            commands: Vec::new(),
            uniform_buffer,
        }
    }

    pub fn add_command(&mut self, command: Command<TUniform>) {
        self.commands.push(command);
    }

    pub fn render(&mut self) {
        let mut vert_shader: Option<&dyn VertexProgram<TUniform>> = None;
        let mut frag_shader: Option<&dyn FragmentProgram<TUniform>> = None;
        let mut render_target: Option<&mut dyn RenderTarget> = None;
        let mut depth_buffer: Option<&mut DepthBuffer> = None;

        for command in &self.commands {
            match command {
                Command::DrawMesh { mesh, transform } => {
                    let vert_shader = vert_shader.expect("No vertex shader set");
                    let frag_shader = frag_shader.expect("No fragment shader set");
                    let target = render_target
                        .as_mut()
                        .expect("No render target set")
                        .deref_mut();
                    let depth_buffer = depth_buffer
                        .as_mut()
                        .expect("No depth buffer set")
                        .deref_mut();

                    self.draw_mesh(
                        mesh,
                        transform,
                        vert_shader,
                        frag_shader,
                        depth_buffer,
                        target,
                    );
                }
                Command::SetRenderTarget { mut target } => render_target = Some(target.deref_mut()),
                Command::SetDepthBuffer { depth_buffer } => todo!(),
                Command::SetVertexShader { shader } => todo!(),
                Command::SetFragmentShader { shader } => todo!(),
            }
        }
    }

    fn draw_mesh(
        &self,
        mesh: &Mesh,
        transform: &Mat4,
        vert_shader: &dyn VertexProgram<TUniform>,
        frag_shader: &dyn FragmentProgram<TUniform>,
        depth: &mut DepthBuffer,
        target: &mut dyn RenderTarget,
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
}
