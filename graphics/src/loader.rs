use crate::render::mesh::Mesh;
use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn load_mesh(path: &Path) -> Result<Mesh> {
    let acc_v = vec![];
    let acc_vn = vec![];
    let acc_vt = vec![];

    let positions = vec![];
    let normals = vec![];
    let uvs = vec![];
    let indices = vec![];

    let reader = BufReader::new(File::open(path)?);

    for line in reader.lines() {
        let line = line?;

        let parts = line
            .split_whitespace()
            .map(|p| p.trim())
            .collect::<Vec<_>>();
    }
}
