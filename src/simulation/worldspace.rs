#![allow(clippy::explicit_counter_loop)]
use std::sync::{mpsc::Sender, Arc, Mutex};

use sfml::system::Vector2f;

use crate::{spacebody::*, transmission::SimulationEvent};

use rayon::prelude::*;

#[allow(unused)]
pub struct WorldSpace {
    bodies: Vec<SpaceBody>,
    highlighted_id: Option<usize>,
}
impl WorldSpace {
    #[allow(unused)]
    pub fn new() -> WorldSpace {
        WorldSpace {
            bodies: Vec::new(),
            highlighted_id: None,
        }
    }
    fn update_positions(&mut self, sender: Sender<SimulationEvent>, dt: f32) {
        //updating positions of each body
        self.bodies
            .par_iter_mut()
            .enumerate()
            .for_each_with(sender, |sen, tuple| {
                //so that each can send to the other thread
                let body = tuple.1;
                sen.send(SimulationEvent::Move {
                    id: body.id,
                    idx: tuple.0,
                    pos: Vector2f::new(body.x, body.y),
                    change: Vector2f::new(body.xv, body.yv),
                })
                .unwrap();
                body.x += body.xv * dt;
                body.y += body.yv * dt;
            });
    }
    fn update_velocity(&mut self, dt: f32) {
        self.bodies.par_iter_mut().for_each(|body| {
            body.xv += body.ax;
            body.yv += body.ay;
        });
    }
    fn update_accelerations(&mut self, g: f32, softening: f32) {
        let mut ax = 0;
        let mut ay = 0;
    }
}
