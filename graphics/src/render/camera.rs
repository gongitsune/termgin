use glam::Mat4;

#[derive(Debug)]
pub struct Camera {
    pub projection: Mat4,
    pub view: Mat4,
}
