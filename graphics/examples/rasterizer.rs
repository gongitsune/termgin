use glam::{Vec2, Vec3A, Vec4};
use graphics::raster::{
    depth::DepthBuffer,
    raster::triangle,
    shader::{FragmentProgram, VertexProgram},
    target::RenderTarget,
    vertex::Vertex,
};
use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};

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
        *output = varying.pos;
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
            "\x1b[{};{}H\x1b[48;2;{};{};{}m \x1b[0m",
            y + 1,
            x + 1,
            color.x as usize * 255,
            color.y as usize * 255,
            color.z as usize * 255
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
            width: ws.ws_row as usize,
            height: ws.ws_col as usize,
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
            pos: Vec4::new(0.1, 0.0, 0.0, 1.0),
            normal: Vec3A::new(0.0, 0.0, 0.0),
            uv: Vec2::new(0.0, 0.0),
        },
        Vertex {
            pos: Vec4::new(0.0, 0.9, 0.0, 1.0),
            normal: Vec3A::new(0.0, 0.0, 0.0),
            uv: Vec2::new(0.0, 0.0),
        },
        Vertex {
            pos: Vec4::new(0.9, 0.5, 0.0, 1.0),
            normal: Vec3A::new(0.0, 0.0, 0.0),
            uv: Vec2::new(0.0, 0.0),
        },
    ];

    print!("\x1b[2J");
    triangle(
        &vert_shader,
        &frag_shader,
        &mut depth,
        &mut term,
        &Uniform {},
        &verts,
    );
}
