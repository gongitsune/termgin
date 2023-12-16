use anyhow::Result;
use glam::{vec3, vec4, Mat4, Vec4};
use graphics::{
    loader::{load_mesh, load_texture},
    material::{
        tex_mat::{self, TexMat},
        Material,
    },
    raster::depth::DepthBuffer,
    render::{camera::Camera, renderer::Renderer, texture::Texture},
    terminal::{color::ColorTerminal, Terminal},
};
use std::{f32::consts::FRAC_PI_2, path::Path};

fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("trace"));

    let mut term = ColorTerminal::new();

    let aspect = term.width() as f32 / term.height() as f32;
    let camera = Camera {
        projection: Mat4::perspective_rh(FRAC_PI_2, aspect, 0.01, 100.0),
        view: Mat4::look_at_rh(
            vec3(0.3, 0.3, 1.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, -1.0, 0.0),
        ),
    };

    let main_tex = load_texture(Path::new("./data/BaseColor.png"))?;

    let scale = Mat4::from_scale(vec3(0.4, 0.4, 0.4));
    let uniform = tex_mat::Uniform {
        world: Mat4::from_rotation_y(0.0),
        view: camera.view * scale,
        projection: camera.projection,
        main_tex,
    };
    let renderer = Renderer::new(uniform);
    let mut depth = DepthBuffer::new(term.width(), term.height());
    let mut target = Texture::new(term.width(), term.height());

    let mesh = load_mesh(Path::new("./data/cube.obj"))?;
    dbg!(&mesh);
    let mat = TexMat::new();

    renderer.clear(&mut target, &mut depth, Vec4::ZERO);
    renderer.draw_mesh(
        &mesh,
        mat.vert_shader(),
        mat.frag_shader(),
        &mut depth,
        &mut target,
    );
    term.present(&target);

    println!("Press any key to continue...");
    target.clear(&vec4(1.0, 0.0, 0.0, 1.0));
    target.load_from_depth(&depth);
    term.present(&target);

    Ok(())
}
