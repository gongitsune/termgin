use glam::{Vec2, Vec3A, Vec4};
use graphics::{
    raster::{depth::DepthBuffer, raster::triangle, target::RenderTarget, vertex::Vertex},
    shader::{FragmentProgram, VertexProgram},
};
use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use log::info;

struct VertShader {}
struct FragShader {}
struct TerminalTarget {
    width: usize,
    height: usize,
}
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
impl RenderTarget for TerminalTarget {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn set(&mut self, x: usize, y: usize, color: &Vec4) {
        print!(
            "\x1b[{};{}H\x1b[48;2;{};{};{}m ",
            y + 1,
            x + 1,
            (color.x * 255.) as usize,
            (color.y * 255.) as usize,
            (color.z * 255.) as usize
        );
    }
}
impl TerminalTarget {
    fn new() -> Self {
        let mut ws: winsize = unsafe { std::mem::zeroed() };
        ws = if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut ws) } == -1 {
            None
        } else {
            Some(ws)
        }
        .unwrap();
        Self {
            width: ws.ws_col as usize,
            height: ws.ws_row as usize,
        }
    }
}

fn main() {
    env_logger::init();

    let mut term = TerminalTarget::new();
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

    print!("\x1b[2J");
    triangle(
        &vert_shader,
        &frag_shader,
        &mut depth,
        &mut term,
        &Uniform {},
        &verts,
    );
    print!("\x1b[0m");
}
