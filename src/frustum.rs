use nalgebra::Matrix4;
use transform::Transform;

pub struct Frustum {
    pub transform: Transform,
    pub projection: Matrix4<f32>,
    pub view_projection: Matrix4<f32>
}

impl Frustum {
    pub fn new(transform: Transform, projection: Matrix4<f32>) -> Frustum {
        Frustum {
            transform,
            projection,
            view_projection: projection * transform.inverse().to_homogeneous()
        }
    }
}