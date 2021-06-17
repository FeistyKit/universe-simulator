#![allow(clippy::explicit_counter_loop)]
use std::sync::{mpsc::Sender, Arc, Mutex};

use sfml::system::Vector2f;

use crate::{spacebody::*, transmission::SimulationEvent};

use rayon::prelude::*;

#[allow(unused)]
pub struct WorldSpace {
    bodies: Vec<SpaceBody>,
    highlighted_id: Option<usize>,
    stopped: bool,
    g: f32,
    softening: f32,
    prepared_id: usize, //this is how I will have it automatically assign an id to each body
}
impl WorldSpace {
    #[allow(unused)]
    pub fn new() -> WorldSpace {
        WorldSpace {
            bodies: Vec::new(),
            highlighted_id: None,
            stopped: false,
            g: 50.0,
            softening: 30.0,
            prepared_id: 0,
        }
    }
    fn update_positions(&mut self, sender: Sender<SimulationEvent>, dt: f32) {
        //updating positions of each body
        self.bodies
            .par_iter_mut()
            .enumerate() //getting the index of each element
            .for_each_with(sender, |sen, tuple| {
                //cloning the sender so that it can be sent to each thread to send to the graphical/main thread
                let body = tuple.1;
                sen.send(SimulationEvent::Move {
                    id: body.id.unwrap(),
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
            body.xv += body.ax * dt;
            body.yv += body.ay * dt;
        });
    }
    pub fn clear(&mut self, sender: &mut Sender<SimulationEvent>) {
        self.bodies = Vec::new();
        sender.send(SimulationEvent::Clear).unwrap();
    }
    fn update_accelerations(&mut self) {
        let g = self.g;
        let softening = self.softening;
        for idx in 0..self.bodies.len() {
            let ax = Arc::new(Mutex::new(0.0));
            let ay = Arc::new(Mutex::new(0.0));
            let id = self.bodies[idx].id;
            let body_x = self.bodies[idx].x;
            let body_y = self.bodies[idx].y; //these are so that it's not double-borrowed
            self.bodies.par_iter()
            .for_each(|other| {
                if other.id == id { //so that it doesn't do gravity for itself, which causes extreme errors
                    return;
                }
                let dx = body_x - other.x;
                let dy = body_y - other.y;
                let squared = dx * dx + dy * dy;
                let force = (g * other.mass) / (squared * (squared + softening /* This is to correct the idea that every body is simply a point*/).sqrt());
                *ax.lock().unwrap() += dx * force;
                *ay.lock().unwrap() += dy * force;
            });
            let mut body = &mut self.bodies[idx];
            body.ax = *ax.lock().unwrap() * -1.0; //for a reason I don't really understand the accelerations are the opposite of what they should be, so this is the fix
            body.ay = *ay.lock().unwrap() * -1.0;
        }
    }
    #[allow(unused)]
    pub fn stop(&mut self) {
        self.stopped = true;
    }
    #[allow(unused)]
    pub fn unstop(&mut self) {
        self.stopped = false;
    }
    pub fn update_advance(&mut self, dt: f32, sender: Sender<SimulationEvent>) {
        if !self.stopped {
            //so time can be "frozen"
            self.update_accelerations();
            self.update_velocity(dt);
            self.update_positions(sender, dt);
        }
    }
    pub fn add_body(&mut self, mut body: SpaceBody, sender: &mut Sender<SimulationEvent>) {
        if body.id.is_none() {
            body.id = Some(self.prepared_id);
            self.prepared_id += 1;
        }
        sender
            .send(SimulationEvent::Add {
                id: body.id.unwrap(),
                pos: Vector2f::new(body.x, body.y),
                color: body.colour,
                size: body.radius,
            })
            .unwrap();
        self.bodies.push(body);
    }
}
