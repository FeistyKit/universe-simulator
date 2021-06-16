#![allow(unused)]
use sfml::{
    graphics::Color,
    system::{Vector2, Vector2f},
};
#[derive(Debug)]
pub enum SimulationEvent {
    //events that can be sent to the main thread for graphics processing
    Move {
        id: usize,
        idx: usize,
        pos: Vector2f,
        change: Vector2f,
    },
    Delete {
        id: usize,
        idx: usize,
    },
    Add {
        id: usize,
        pos: Vector2f,
        color: (u8, u8, u8),
        size: f32,
    },
}
#[derive(Debug)]
pub enum InputEvent {
    //Events from input to be sent to the simulation thread
    LeftClick {
        screen_pos: Vector2<i32>,
        pos: Vector2f,
        highlighted_colour: Color,
        highlighted_size: f32,
        highlighted_mass: f32,
    },
    ShutDown,
}
impl InputEvent {
    pub fn unwrap_lcos(self) -> (Vector2<i32>, Vector2f, Color, f32, f32) {
        match self {
            InputEvent::LeftClick {
                screen_pos,
                pos,
                highlighted_colour,
                highlighted_size,
                highlighted_mass,
            } => (
                screen_pos,
                pos,
                highlighted_colour,
                highlighted_size,
                highlighted_mass,
            ),
            _ => panic!("{:?} is not lcos!", self),
        }
    }
}
