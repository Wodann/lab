use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied};
use std::sync::mpsc::Sender;

use nalgebra::Vector2;
use winit::{MouseButton};

pub use winit::ElementState as ElementState;
pub use winit::VirtualKeyCode as VirtualKeyCode;
pub use winit::ModifiersState as ModifiersState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyboardInput {
    pub state: ElementState,
    pub key: VirtualKeyCode,
    pub modifiers: ModifiersState
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MouseInput {
    pub state: ElementState,
    pub button: MouseButton,
    pub modifiers: ModifiersState
}

impl KeyboardInput {
    pub fn new(state: ElementState, key: VirtualKeyCode, modifiers: ModifiersState) -> KeyboardInput {
        KeyboardInput {
            state,
            key,
            modifiers
        }
    }
}

impl MouseInput {
    pub fn new(state: ElementState, button: MouseButton, modifiers: ModifiersState) -> MouseInput {
        MouseInput {
            state,
            button,
            modifiers
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputEventDesc {
    KeyboardInput(KeyboardInput),
    MouseInput(MouseInput),
    MouseMoved
}

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    KeyboardInput(KeyboardInput),
    MouseInput(MouseInput),
    MouseMoved(Vector2<f32>),
}

pub struct InputHandler {
    subscribers: HashMap<InputEventDesc, Vec<Sender<InputEvent>>>,
    keystate: [ElementState; 161],  // [TODO] Derive the length of winit::VirtualKeyCode automatically
    last_mouse_pos: Option<Vector2<f32>>,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            subscribers: HashMap::new(),
            keystate: [ElementState::Released; 161],
            last_mouse_pos: None
        }
    }

    pub fn handle_keyboard_input(&mut self, input: KeyboardInput) -> &mut Self {
        let event = InputEventDesc::KeyboardInput(input.clone());
        if let Some(keystate) = self.keystate.get_mut(input.key as usize) {
            if *keystate != input.state {
                *keystate = input.state;

                if let Occupied(mut subscribers) = self.subscribers.entry(event) {
                    subscribers.get_mut().retain(|subscriber|
                        subscriber.send(InputEvent::KeyboardInput(input.clone())).is_ok()
                    );
                }
            }
        }
        self
    }

    pub fn handle_mouse_input(&mut self, input: MouseInput) -> &mut Self {
        let event = InputEventDesc::MouseInput(input.clone());
        if let Occupied(mut subscribers) = self.subscribers.entry(event) {
            subscribers.get_mut().retain(|subscriber|
                subscriber.send(InputEvent::MouseInput(input.clone())).is_ok()
            );
        }
        self
    }

    pub fn handle_mouse_move(&mut self, pos: Vector2<f32>) -> &mut Self {
        let mouse_delta = pos - self.last_mouse_pos.get_or_insert(pos).clone();
        if let Occupied(mut subscribers) = self.subscribers.entry(InputEventDesc::MouseMoved) {
            subscribers.get_mut().retain(|subscriber|
                subscriber.send(InputEvent::MouseMoved(mouse_delta.clone())).is_ok()
            );
        }
        self
    }

    pub fn subscribe_to_input(&mut self, input: InputEventDesc, sender: Sender<InputEvent>) -> &mut Self {
        self.subscribers.entry(input).or_insert(Vec::new()).push(sender);
        self
    }
}