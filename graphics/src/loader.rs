use crate::{raster::vertex::Vertex, render::mesh::Mesh};
use anyhow::{Ok, Result};
use glam::{vec2, vec3a, vec4};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn load_mesh(path: &Path) -> Result<Mesh> {
    let mut acc_v = vec![];
    let mut acc_vn = vec![];
    let mut acc_vt = vec![];

    let mut positions = vec![];
    let mut normals = vec![];
    let mut uvs = vec![];
    let mut indices: Vec<u32> = vec![];

    let reader = BufReader::new(File::open(path)?);

    for line in reader.lines() {
        let line = line?;

        let parts = line
            .split_whitespace()
            .map(|p| p.trim())
            .collect::<Vec<_>>();
        if parts.len() > 0 {
            match parts[0] {
                "v" => {
                    let x = parts[1].parse::<f32>()?;
                    let y = parts[2].parse::<f32>()?;
                    let z = parts[3].parse::<f32>()?;
                    acc_v.push(vec4(x, y, z, 1.0));
                }
                "vn" => {
                    let x = parts[1].parse::<f32>()?;
                    let y = parts[2].parse::<f32>()?;
                    let z = parts[3].parse::<f32>()?;
                    acc_vn.push(vec3a(x, y, z));
                }
                "vt" => {
                    let x = parts[1].parse::<f32>()?;
                    let y = parts[2].parse::<f32>()?;
                    acc_vt.push(vec2(x, y));
                }
                "f" => {
                    for i in 1..=3 {
                        let face = parts[i].split('/').collect::<Vec<_>>();
                        let i_v = face[0].parse::<usize>()? - 1;
                        let i_vt = face[1].parse::<usize>()? - 1;
                        let i_vn = face[2].parse::<usize>()? - 1;
                        assert!(i_v < acc_v.len());
                        assert!(i_vt < acc_vt.len());
                        assert!(i_vn < acc_vn.len());
                        positions.push(acc_v[i_v]);
                        uvs.push(acc_vt[i_vt]);
                        normals.push(acc_vn[i_vn]);
                        indices.push(indices.len() as u32);
                    }
                }
                _ => (),
            }
        }
    }

    let vertices = positions
        .iter()
        .zip(normals.iter())
        .zip(uvs.iter())
        .map(|((p, n), uv)| Vertex::new(*p, *n, *uv))
        .collect::<Vec<_>>();
    return Ok(Mesh { vertices, indices });
}
