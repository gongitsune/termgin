use crate::{
    raster::vertex::VertexTrait,
    render::{mesh::Mesh, texture::Texture},
};
use anyhow::{anyhow, Ok, Result};
use obj::{Obj, TexturedVertex};
use std::{fs::File, io::BufReader, path::Path};

pub fn load_texture(path: &Path) -> Result<Texture> {
    let img = image::open(path)?.flipv();

    Ok(Texture::from(img))
}

fn load_obj<TVertex, F>(reader: BufReader<File>, f: F) -> Result<Mesh<TVertex>>
where
    TVertex: VertexTrait + Default + Copy,
    F: Fn(&TexturedVertex) -> TVertex,
{
    let model: Obj<TexturedVertex> = obj::load_obj(reader)?;

    let vertices = model.vertices.iter().map(f).collect::<Vec<_>>();
    let indices = model.indices.iter().map(|i| *i as u32).collect::<Vec<_>>();

    Ok(Mesh { vertices, indices })
}

pub fn load_mesh<TVertex, F>(path: &Path, f: F) -> Result<Mesh<TVertex>>
where
    TVertex: VertexTrait + Default + Copy,
    F: Fn(&TexturedVertex) -> TVertex,
{
    let reader = BufReader::new(File::open(path)?);

    match path.extension().and_then(|s| s.to_str()) {
        Some("obj") => load_obj(reader, f),
        Some(ext) => Err(anyhow!("unsupported file extension: {}", ext)),
        None => Err(anyhow!("no file extension")),
    }
}
