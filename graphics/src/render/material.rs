use glam::Mat4;

pub struct ConstantBuffer {
    pub world: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
}

pub struct TextureBuffer {
    // pub main_tex: Texture
}
