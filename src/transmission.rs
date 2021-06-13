#![allow(unused)]
use sfml::{graphics::Color, system::Vector2f};
#[derive(Debug)]
enum SimulationEvent {
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
    },
}
#[derive(Debug)]
enum InputEvent {
    //Events from input to be sent to the simulation thread
    LeftClickOnSpace {
        screenpos: Vector2f,
        highlighted_colour: Color,
        highlighted_size: f32,
        highlighted_mass: f32,
    },
}
impl InputEvent {
    pub fn unwrap_lcos(self) -> (Vector2f, Color, f32, f32) {
        match self {
            InputEvent::LeftClickOnSpace {
                screenpos,
                highlighted_colour,
                highlighted_size,
                highlighted_mass,
            } => (
                screenpos,
                highlighted_colour,
                highlighted_size,
                highlighted_mass,
            ),
            _ => panic!("{:?} is not lcos!", self),
        }
    }
}
