use anyhow::Result;
use glam::{vec3, vec4, EulerRot, Mat4, Quat, Vec4};
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
use std::{
    f32::consts::{FRAC_PI_2, PI},
    path::Path,
};

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

    let scale = Mat4::from_scale(vec3(0.3, 0.3, 0.3));
    let uniform = tex_mat::Uniform {
        world: Mat4::from_quat(Quat::from_euler(EulerRot::XYZ, 0.4, PI / 12.0 * 4.0, 0.0)),
        view: camera.view * scale,
        projection: camera.projection,
        main_tex,
    };
    let mut renderer = Renderer::new(uniform);
    let mut depth = DepthBuffer::new(term.width(), term.height());
    let mut target = Texture::new(term.width(), term.height());

    let mesh = load_mesh(Path::new("./data/cube.obj"))?;
    let mat = TexMat::new();

    let mut delta = 0.0;
    loop {
        delta += 0.1;

        renderer.uniform_buffer.world = Mat4::from_quat(Quat::from_euler(
            EulerRot::XYZ,
            PI / 12.0 * 4.0 * delta,
            PI / 12.0 * 4.0 * delta,
            0.0,
        ));

        renderer.clear(&mut target, &mut depth, Vec4::ZERO);
        renderer.draw_mesh(
            &mesh,
            mat.vert_shader(),
            mat.frag_shader(),
            &mut depth,
            &mut target,
        );
        term.present(&target);
    }

    //
    // target.clear(&vec4(1.0, 0.0, 0.0, 1.0));
    // target.load_from_depth(&depth);
    // term.present(&target);
}
