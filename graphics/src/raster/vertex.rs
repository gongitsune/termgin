use glam::{Vec2, Vec3A, Vec4};

#[derive(Debug, Default, Copy, Clone)]
pub struct Vertex {
    pub pos: Vec4,
    pub normal: Vec3A,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new(pos: Vec4, normal: Vec3A, uv: Vec2) -> Self {
        Self { pos, normal, uv }
    }

    pub fn correct_mut(&mut self, w: f32) {
        self.pos /= w;
        self.normal /= w;
        self.uv /= w;
    }

    pub fn interpolate(v: &[Vertex; 3], cw: &[f32; 3], w: f32, output: &mut Self) {
        output.pos = v[0].pos * cw[0] + v[1].pos * cw[1] + v[2].pos * cw[2];
        output.normal = v[0].normal * cw[0] + v[1].normal * cw[1] + v[2].normal * cw[2];
        output.uv = v[0].uv * cw[0] + v[1].uv * cw[1] + v[2].uv * cw[2];
        output.correct_mut(w);
    }
}
