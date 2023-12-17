use crate::{
    raster::vertex::Vertex,
    render::{mesh::Mesh, texture::Texture},
};
use anyhow::{anyhow, Ok, Result};
use glam::{vec2, vec4, Vec3A};
use obj::{Obj, TexturedVertex};
use std::{fs::File, io::BufReader, path::Path};

pub fn load_texture(path: &Path) -> Result<Texture> {
    let img = image::open(path)?.flipv();

    Ok(Texture::from(img))
}

fn load_obj(reader: BufReader<File>) -> Result<Mesh> {
    let model: Obj<TexturedVertex> = obj::load_obj(reader)?;

    let vertices = model
        .vertices
        .iter()
        .map(|v| Vertex {
            pos: vec4(v.position[0], v.position[1], v.position[2], 1.0),
            normal: Vec3A::from_array(v.normal),
            uv: vec2(v.texture[0], v.texture[1]),
        })
        .collect::<Vec<_>>();
    let indices = model.indices.iter().map(|i| *i as u32).collect::<Vec<_>>();

    Ok(Mesh { vertices, indices })
}

fn load_gltf(reader: BufReader<File>) -> Result<Mesh> {
    let gltf = gltf::Gltf::from_reader(reader)?;

    let gltf_mesh = gltf.meshes().next().ok_or(anyhow!("no meshes"))?;
    println!("gltf mesh: {:?}", gltf_mesh);
    Err(anyhow!("not implemented"))
}

pub fn load_mesh(path: &Path) -> Result<Mesh> {
    let reader = BufReader::new(File::open(path)?);

    match path.extension().and_then(|s| s.to_str()) {
        Some("obj") => load_obj(reader),
        Some("gltf") => load_gltf(reader),
        Some(ext) => Err(anyhow!("unsupported file extension: {}", ext)),
        None => Err(anyhow!("no file extension")),
    }
}
