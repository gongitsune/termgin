use super::Material;
use crate::{
    raster::vertex::SimpleVertex,
    render::texture::Texture,
    shader::tex_mat::{TexMatFragShader, TexMatVertShader},
};
use glam::Mat4;

pub struct Uniform {
    pub world: Mat4,
    pub view: Mat4,
    pub projection: Mat4,

    pub main_tex: Texture,
}

pub struct TexMat {
    vert_shader: TexMatVertShader,
    frag_shader: TexMatFragShader,
}

impl Default for TexMat {
    fn default() -> Self {
        Self::new()
    }
}

impl TexMat {
    pub fn new() -> Self {
        Self {
            vert_shader: TexMatVertShader {},
            frag_shader: TexMatFragShader {},
        }
    }
}

impl Material<Uniform, TexMatVertShader, TexMatFragShader, SimpleVertex> for TexMat {
    fn vert_shader(&self) -> &TexMatVertShader {
        &self.vert_shader
    }

    fn frag_shader(&self) -> &TexMatFragShader {
        &self.frag_shader
    }
}
