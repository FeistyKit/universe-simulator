#![allow(clippy::explicit_counter_loop)]
use std::sync::mpsc::Sender;

use sfml::system::Vector2f;

use crate::{spacebody::*, transmission::SimulationEvent};

use rayon::prelude::*;

#[allow(unused)]
pub struct WorldSpace {
    bodies: SpaceBodyVec,
    highlighted_id: Option<usize>,
}
impl WorldSpace {
    #[allow(unused)]
    pub fn new() -> WorldSpace {
        WorldSpace {
            bodies: SpaceBodyVec::new(),
            highlighted_id: None,
        }
    }
    fn update_positions(&mut self, sender: &mut Sender<SimulationEvent>, dt: f32) {
        //updating positions of each body
        let mut idx = 0;
        for (x, y, xv, yv, id) in soa_zip!(&mut self.bodies, [mut x, mut y, xv, yv, id]) {
            //this is so the computer doesn't have to drudge up every field of the struct when it doesn't need to. You'll see this a lot
            *x += xv * dt;
            *y += yv * dt;
            sender
                .send(SimulationEvent::Move {
                    id: *id,
                    idx,
                    pos: Vector2f::new(*x, *y),
                    change: Vector2f::new(*xv, *yv),
                })
                .unwrap();
            idx += 1;
        }
    }
    fn update_velocity(&mut self, dt: f32) {
        for (xv, yv, ax, ay) in soa_zip!(&mut self.bodies, [mut xv, mut yv, ax, ay]) {
            *xv += *ax * dt;
            *yv += *ay * dt;
        }
    }
    fn update_accelerations(&mut self, g: f32, softening: f32) {
        let mut ax = 0;
        let mut ay = 0;
    }
}
