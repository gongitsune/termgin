use super::{
    depth::DepthBuffer,
    shader::{FragmentProgram, VertexProgram},
    target::RenderTarget,
    vertex::Vertex,
};
use glam::{vec2, Vec2, Vec4};

fn edge(v0: &[&Vec2; 3]) -> f32 {
    (v0[1].x - v0[0].x) * (v0[2].y - v0[0].y) - (v0[2].x - v0[0].x) * (v0[1].y - v0[0].y)
}

fn draw_line<TUniform, TFrag, TTarget>(
    frag_shader: &TFrag,
    depth: &mut DepthBuffer,
    target: &mut TTarget,
    uniform: &TUniform,
    clippos: &[&Vec2; 3],
    varying: &[Vertex; 3],
    corrected_z: &[f32; 3],
    min_x: usize,
    max_x: usize,
    y: usize,
) where
    TFrag: FragmentProgram<TUniform>,
    TTarget: RenderTarget,
{
    if y >= target.height() {
        return;
    }

    let min_x = min_x.max(0);
    let max_x = max_x.min(target.width());

    let edge_value = edge(clippos);

    let mut frag_varying = Vertex::default();
    let mut frag_color = Vec4::default();
    for x in min_x..max_x {
        let coord = vec2(x as f32, y as f32);

        let weight = [
            edge(&[clippos[2], clippos[1], &coord]) / edge_value,
            edge(&[clippos[0], clippos[2], &coord]) / edge_value,
            edge(&[clippos[1], clippos[0], &coord]) / edge_value,
        ];

        let calc_depth =
            weight[0] * corrected_z[0] + weight[1] * corrected_z[1] + weight[2] * corrected_z[2];

        if calc_depth <= depth.get(x, y) {
            depth.set(x, y, calc_depth);
            Vertex::interpolate(varying, &weight, calc_depth, &mut frag_varying);
            frag_shader.main(uniform, &frag_varying, &mut frag_color);
            target.set(x, y, &frag_color);
        }
    }
}

fn calc_x_scan_range(y: usize, ordered: &[&Vec2; 4]) -> (usize, usize) {
    let gradient_0 = if ordered[0].y != ordered[1].y {
        (y as f32 - ordered[0].y) / (ordered[1].y - ordered[0].y)
    } else {
        1.0
    };
    let gradient_1 = if ordered[2].y != ordered[3].y {
        (y as f32 - ordered[2].y) / (ordered[3].y - ordered[2].y)
    } else {
        1.0
    };

    let min_x = ordered[0].x + (ordered[1].x - ordered[0].x) * gradient_0.clamp(0., 1.);
    let max_x = ordered[2].x + (ordered[3].x - ordered[2].x) * gradient_1.clamp(0., 1.);

    (min_x as usize, max_x as usize)
}

fn draw_triangle<TUniform, TFrag, TTarget>(
    frag_shader: &TFrag,
    depth: &mut DepthBuffer,
    target: &mut TTarget,
    uniform: &TUniform,
    varying: &[Vertex; 3],
    clippos: &[&Vec2; 3],
    corrected_z: &[f32; 3],
) where
    TFrag: FragmentProgram<TUniform>,
    TTarget: RenderTarget,
{
    let mut ordered = [clippos[0], clippos[1], clippos[2]];
    ordered.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    for y in ordered[0].y as usize..=ordered[2].y as usize {
        let (min_x, max_x) = if y < ordered[1].y as usize {
            calc_x_scan_range(y, &[ordered[0], ordered[2], ordered[0], ordered[1]])
        } else {
            calc_x_scan_range(y, &[ordered[0], ordered[2], ordered[1], ordered[2]])
        };

        draw_line(
            frag_shader,
            depth,
            target,
            uniform,
            clippos,
            varying,
            corrected_z,
            min_x,
            max_x,
            y,
        );
    }

    for y in ordered[0].y as usize..=ordered[2].y as usize {
        let (min_x, max_x) = if y < ordered[1].y as usize {
            calc_x_scan_range(y, &[ordered[0], ordered[1], ordered[0], ordered[2]])
        } else {
            calc_x_scan_range(y, &[ordered[1], ordered[2], ordered[0], ordered[2]])
        };

        draw_line(
            frag_shader,
            depth,
            target,
            uniform,
            clippos,
            varying,
            corrected_z,
            min_x,
            max_x,
            y,
        );
    }
}

pub fn triangle<TUniform, TVert, TFrag, TTarget>(
    vert_shader: &TVert,
    frag_shader: &TFrag,
    depth: &mut DepthBuffer,
    target: &mut TTarget,
    uniform: &TUniform,
    verts: &[&Vertex; 3],
) where
    TVert: VertexProgram<TUniform>,
    TFrag: FragmentProgram<TUniform>,
    TTarget: RenderTarget,
{
    let width = target.width() as f32;
    let height = target.height() as f32;
    let half_width = width as f32 / 2.0;
    let half_height = height as f32 / 2.0;

    let mut varying = [Vertex::default(); 3];
    let mut pos = [Vec4::default(); 3];

    // vert shader
    for i in 0..3 {
        vert_shader.main(uniform, verts[i], &mut varying[i], &mut pos[i]);
    }

    if pos.iter().all(|v| v.w < 0.0) {
        return;
    }

    let clippos = [
        &vec2(
            pos[0].x / pos[0].w * width + half_width,
            -pos[0].y / pos[0].w * height + half_height,
        ),
        &vec2(
            pos[1].x / pos[1].w * width + half_width,
            -pos[1].y / pos[1].w * height + half_height,
        ),
        &vec2(
            pos[2].x / pos[2].w * width + half_width,
            -pos[2].y / pos[2].w * height + half_height,
        ),
    ];

    for i in 0..3 {
        varying[i].correct_mut(pos[i].w);
    }

    if edge(&clippos) >= 0. {
        draw_triangle(
            frag_shader,
            depth,
            target,
            uniform,
            &varying,
            &clippos,
            &[1. / pos[0].w, 1. / pos[1].w, 1. / pos[2].w],
        );
    }
}
