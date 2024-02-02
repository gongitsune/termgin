use crate::raster::vertex::VertexTrait;

#[derive(Debug)]
pub struct Mesh<TVertex>
where
    TVertex: VertexTrait + Default + Copy,
{
    pub vertices: Vec<TVertex>,
    pub indices: Vec<u32>,
}
