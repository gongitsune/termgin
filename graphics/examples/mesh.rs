use anyhow::Result;
use glam::{vec3, EulerRot, Mat4, Quat, Vec4};
use graphics::{
    loader::load_mesh,
    material::{
        tex_mat::{self, TexMat},
        Material,
    },
    raster::{depth::DepthBuffer, target::RenderTarget},
    render::{camera::Camera, renderer::Renderer, texture::Texture},
    terminal::{color::ColorTerminal, Terminal},
};
use std::{f32::consts::PI, path::Path};

fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("trace"));

    let mut term = ColorTerminal::new();

    let aspect = term.width() as f32 / term.height() as f32;
    let camera = Camera {
        projection: Mat4::perspective_lh(PI / 2.0, aspect, 0.1, 1000.0),
        view: Mat4::look_at_lh(
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 1.0, 0.0),
        ),
    };

    let mut main_tex = Texture::new(1, 1);
    main_tex.clear(&Vec4::new(1., 0., 0., 1.));
    let uniform = tex_mat::Uniform {
        world: Mat4::from_scale_rotation_translation(
            vec3(1., 1., 1.),
            Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
            vec3(0., 0., -2.),
        ),
        view: camera.view,
        projection: camera.projection,
        main_tex,
    };
    let renderer = Renderer::new(uniform);
    let mut depth = DepthBuffer::new(term.width(), term.height());
    let mut target = Texture::new(term.width(), term.height());

    let mesh = load_mesh(Path::new("./data/books.obj"))?;
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

    Ok(())
}
