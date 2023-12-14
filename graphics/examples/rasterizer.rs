use glam::{Vec2, Vec3A, Vec4};
use graphics::{
    raster::{depth::DepthBuffer, raster::triangle, vertex::Vertex},
    render::texture::Texture,
    shader::{FragmentProgram, VertexProgram},
    terminal::{color::ColorTerminal, Terminal},
};
use log::info;

struct VertShader {}
struct FragShader {}
struct Uniform {}
impl VertexProgram<Uniform> for VertShader {
    fn main(&self, _: &Uniform, vertex: &Vertex, varying: &mut Vertex, output: &mut Vec4) {
        varying.pos = vertex.pos;
        varying.normal = vertex.normal;
        varying.uv = vertex.uv;
        *output = vertex.pos;
    }
}
impl FragmentProgram<Uniform> for FragShader {
    fn main(&self, _: &Uniform, varying: &Vertex, output: &mut Vec4) {
        *output = varying.pos + 0.5;
    }
}

fn main() {
    env_logger::init();

    let mut term = ColorTerminal::new();
    let mut target = Texture::new(term.width(), term.height());
    let mut depth = DepthBuffer::new(term.width(), term.height());

    let vert_shader = VertShader {};
    let frag_shader = FragShader {};

    let verts = [
        Vertex {
            pos: Vec4::new(-0.5, 0.0, 0.0, 1.0),
            normal: Vec3A::new(0.0, 0.0, 0.0),
            uv: Vec2::new(0.0, 0.0),
        },
        Vertex {
            pos: Vec4::new(0.0, 0.5, 0.0, 1.0),
            normal: Vec3A::new(0.0, 0.0, 0.0),
            uv: Vec2::new(0.0, 0.0),
        },
        Vertex {
            pos: Vec4::new(0.5, 0.0, 0.0, 1.0),
            normal: Vec3A::new(0.0, 0.0, 0.0),
            uv: Vec2::new(0.0, 0.0),
        },
    ];

    info!("term size: {} x {}", term.width(), term.height());

    target.clear(&Vec4::ZERO);
    triangle(
        &vert_shader,
        &frag_shader,
        &mut depth,
        &mut target,
        &Uniform {},
        &verts,
    );
    term.present(&target);
}
