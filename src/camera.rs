use nalgebra::{Matrix4, Translation3, UnitQuaternion, Vector3};
use frustum::Frustum;
use transform::Transform;

pub struct Camera {
    transform: Transform,

    fov: f32,
    near: f32,
    far: f32
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            transform: Transform::identity(),
            fov: 1.0,
            near: 0.1,
            far: 10000.0
        }
    }

    pub fn set_near(&mut self, near: f32) -> &mut Self {
        self.near = near;
        self
    }

    pub fn set_far(&mut self, far: f32) -> &mut Self {
        self.far = far;
        self
    }

    pub fn set_field_of_view(&mut self, fov: f32) -> &mut Self {
        self.fov = fov;
        self
    }

    pub fn roll_by(&mut self, roll: f32) -> &mut Self {
        self.transform.append_rotation_mut(&UnitQuaternion::from_euler_angles(roll, 0.0, 0.0));
        self
    }

    pub fn pitch_by(&mut self, pitch: f32) -> &mut Self {
        self.transform.append_rotation_mut(&UnitQuaternion::from_euler_angles(0.0, pitch, 0.0));
        self
    }

    pub fn yaw_by(&mut self, yaw: f32) -> &mut Self {
        self.transform.append_rotation_mut(&UnitQuaternion::from_euler_angles(0.0, 0.0, yaw));
        self
    }

    pub fn rotate_by(&mut self, rotation: &UnitQuaternion<f32>) -> &mut Self {
        self.transform.append_rotation_mut(&rotation);
        self
    }

    pub fn translate_by(&mut self, translation: &Vector3<f32>) -> &mut Self {
        self.transform.append_translation_mut(&Translation3::from(*translation));
        self
    }

    pub fn frustum(&self, aspect_ratio: f32) -> Frustum {
        Frustum::new(
            self.transform.clone(),
            Matrix4::new_perspective(aspect_ratio, self.fov, self.near, self.far)
        )
    }
}