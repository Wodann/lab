extern crate nalgebra_glm as glm;

use std::sync::mpsc;

use nalgebra::{UnitQuaternion, Vector3};

use camera::Camera;
use input_handler::{ElementState, InputEvent, InputEventDesc, InputHandler, ModifiersState, MouseButton, MouseInput};

pub struct CameraController {
    input_receiver: mpsc::Receiver<InputEvent>,
    controlling: bool
}

impl CameraController {
    pub fn new(input_handler: &mut InputHandler) -> CameraController {
        let (tx, rx) = mpsc::channel();

        input_handler.subscribe_to_input(InputEventDesc::MouseInput(MouseInput::new(
            ElementState::Pressed,
            MouseButton::Left,
            ModifiersState::default()
        )), tx.clone());
        input_handler.subscribe_to_input(InputEventDesc::MouseInput(MouseInput::new(
            ElementState::Released,
            MouseButton::Left,
            ModifiersState::default()
        )), tx.clone());
        input_handler.subscribe_to_input(InputEventDesc::MouseMoved, tx.clone());

        CameraController {
            input_receiver: rx,
            controlling: false
        }
    }

    pub fn update(&mut self, camera: &mut Camera) -> &mut Self {
        while let Ok(event) = self.input_receiver.try_recv() {
            match event {
                InputEvent::MouseInput(MouseInput { state, button, ..}) => {
                    if button == MouseButton::Left {
                        self.controlling = state == ElementState::Pressed;
                    }
                },
                InputEvent::MouseMoved(delta) => {
                    if self.controlling {
                        let local_yaw = glm::quat_rotate(&glm::quat_identity(), delta.x * -0.003, &Vector3::y_axis());
                        let local_pitch = glm::quat_rotate(&glm::quat_identity(), delta.y * 0.003, &Vector3::x_axis());

                        camera.rotate_by(&UnitQuaternion::from_quaternion(local_yaw));
                        camera.rotate_by(&UnitQuaternion::from_quaternion(local_pitch));
                    }
                }
                _ => ()
            }
        }
        self
    }
}