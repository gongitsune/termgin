use crate::{
    raster::vertex::Vertex,
    render::{mesh::Mesh, texture::Texture},
};
use anyhow::{anyhow, Ok, Result};
use glam::{vec2, vec3a, vec4};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn load_texture(path: &Path) -> Result<Texture> {
    let img = image::open(path)?.flipv();

    Ok(Texture::from(img))
}

fn parse_obj(path: &Path) -> Result<Mesh> {
    let reader = BufReader::new(File::open(path)?);

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut tex_coords = Vec::new();
    let mut indices = Vec::new();

    let mut vertices = None;

    for line in reader.lines() {
        let line = line?;
        let parts = line.split(' ').map(|v| v.trim()).collect::<Vec<_>>();
        if parts[0] == "v" {
            let x = parts[1].parse::<f32>()?;
            let y = parts[2].parse::<f32>()?;
            let z = parts[3].parse::<f32>()?;
            positions.push(vec4(x, y, z, 1.0));
        } else if parts[0] == "vn" {
            let x = parts[1].parse::<f32>()?;
            let y = parts[2].parse::<f32>()?;
            let z = parts[3].parse::<f32>()?;
            normals.push(vec3a(x, y, z));
        } else if parts[0] == "vt" {
            let x = parts[1].parse::<f32>()?;
            let y = parts[2].parse::<f32>()?;
            tex_coords.push(vec2(x, y));
        } else if parts[0] == "f" {
            if vertices.is_none() {
                vertices = Some(vec![Vertex::default(); positions.len()]);
            }
            let vertices = vertices.as_mut().unwrap();

            for i in 0..3 {
                let face = parts[3 - i]
                    .split('/')
                    .map(|v| v.parse::<usize>().unwrap() - 1)
                    .collect::<Vec<_>>();
                let v = face[0];
                let vt = face[1];
                let vn = face[2];

                vertices[v].pos = positions[v];
                vertices[v].normal = normals[vn];
                vertices[v].uv = tex_coords[vt];
                indices.push(v as u32);
            }
        }
    }

    Ok(Mesh {
        vertices: vertices.ok_or(anyhow!("No vertices found"))?,
        indices,
    })
}

pub fn load_mesh(path: &Path) -> Result<Mesh> {
    parse_obj(path)
}
