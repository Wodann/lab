extern crate nalgebra_glm as glm;

use std::sync::mpsc;

use nalgebra::{UnitQuaternion, Vector2, Vector3};

use camera::Camera;
use input_handler::{ElementState, InputEvent, InputEventDesc, InputHandler, KeyboardInput, ModifiersState, MouseButton, MouseInput, VirtualKeyCode};

pub struct CameraController {
    input_receiver: mpsc::Receiver<InputEvent>,
    movement_x: Option<f32>,
    movement_z: Option<f32>,
    controlling: bool
}

impl CameraController {
    pub fn new(input_handler: &mut InputHandler) -> CameraController {
        let (tx, rx) = mpsc::channel();

        input_handler.subscribe_to_input(InputEventDesc::KeyboardInput(KeyboardInput::new(
            ElementState::Pressed,
            VirtualKeyCode::A,
            ModifiersState::default()
        )), tx.clone());
        input_handler.subscribe_to_input(InputEventDesc::KeyboardInput(KeyboardInput::new(
            ElementState::Released,
            VirtualKeyCode::A,
            ModifiersState::default()
        )), tx.clone());
        input_handler.subscribe_to_input(InputEventDesc::KeyboardInput(KeyboardInput::new(
            ElementState::Pressed,
            VirtualKeyCode::D,
            ModifiersState::default()
        )), tx.clone());
        input_handler.subscribe_to_input(InputEventDesc::KeyboardInput(KeyboardInput::new(
            ElementState::Released,
            VirtualKeyCode::D,
            ModifiersState::default()
        )), tx.clone());
        input_handler.subscribe_to_input(InputEventDesc::KeyboardInput(KeyboardInput::new(
            ElementState::Pressed,
            VirtualKeyCode::S,
            ModifiersState::default()
        )), tx.clone());
        input_handler.subscribe_to_input(InputEventDesc::KeyboardInput(KeyboardInput::new(
            ElementState::Released,
            VirtualKeyCode::S,
            ModifiersState::default()
        )), tx.clone());
        input_handler.subscribe_to_input(InputEventDesc::KeyboardInput(KeyboardInput::new(
            ElementState::Pressed,
            VirtualKeyCode::W,
            ModifiersState::default()
        )), tx.clone());
        input_handler.subscribe_to_input(InputEventDesc::KeyboardInput(KeyboardInput::new(
            ElementState::Released,
            VirtualKeyCode::W,
            ModifiersState::default()
        )), tx.clone());
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
            movement_x: None,
            movement_z: None,
            controlling: false
        }
    }

    pub fn update(&mut self, camera: &mut Camera, dt: f32) -> &mut Self {
        let mut total_mouse_delta : Option<Vector2<f32>> = None;

        while let Ok(event) = self.input_receiver.try_recv() {
            match event {
                InputEvent::KeyboardInput(KeyboardInput { state, key, .. }) => {
                    match key {
                        VirtualKeyCode::A => self.movement_x = self.movement_x.map_or(Some(-1.0), |dir| {
                            if state == ElementState::Pressed { return Some(-1.0) }
                            if dir == -1.0 { return None }
                            Some(dir)
                        }),
                        VirtualKeyCode::D => self.movement_x = self.movement_x.map_or(Some(1.0), |dir| {
                            if state == ElementState::Pressed { return Some(1.0) }
                            if dir == 1.0 { return None }
                            Some(dir)
                        }),
                        VirtualKeyCode::S => self.movement_z = self.movement_z.map_or(Some(1.0), |dir| {
                            if state == ElementState::Pressed { return Some(1.0) }
                            if dir == 1.0 { return None }
                            Some(dir)
                        }),
                        VirtualKeyCode::W => self.movement_z = self.movement_z.map_or(Some(-1.0), |dir| {
                            if state == ElementState::Pressed { return Some(-1.0) }
                            if dir == -1.0 { return None }
                            Some(dir)
                        }),
                        _ => ()
                    };
                },
                InputEvent::MouseInput(MouseInput { state, button, .. }) => {
                    if button == MouseButton::Left {
                        self.controlling = state == ElementState::Pressed;
                    }
                },
                InputEvent::MouseMoved(delta) => {
                    if self.controlling {
                        total_mouse_delta = Some(total_mouse_delta.map_or(delta, |total| total + delta));
                    }
                }
                _ => ()
            }
        }

        let movement_dir = self.movement_x.map_or_else(
            || self.movement_z.map(|z| Vector3::new(0.0, 0.0, z)),
            |x| self.movement_z.map_or_else(
                || Some(Vector3::new(x, 0.0, 0.0)),
                |z| Some(Vector3::new(x, 0.0, z))
            )
        );

        if let Some(dir) = movement_dir {
            camera.local_translate_by(&(dt * dir));
        }

        if let Some(delta) = total_mouse_delta {
            camera.local_yaw_by(delta.x * -0.003);
            camera.local_pitch_by(delta.y * 0.003);
        }

        self
    }
}