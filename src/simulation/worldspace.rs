#![allow(clippy::explicit_counter_loop)]
use std::{
    fs,
    io::Write,
    sync::{mpsc::Sender, Arc, Mutex},
};

use sfml::system::Vector2f;

use crate::{spacebody::*, transmission::SimulationEvent};

use serde::{Deserialize, Serialize};

use rayon::prelude::*;

#[allow(unused)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorldSpace {
    bodies: Vec<SpaceBody>,
    highlighted_id: Option<usize>, //the id of the spacebody that is being "followed" by the camera
    stopped: bool,                 //whether the simulation is paused or not
    g: f32, //the force of gravity. I've got the default as 50.0 right now but some tweaking might be in order
    softening: f32, //a constant to stop a bug where bodies pass over each other and move incredibly quickly. also needs some tweaking
    prepared_id: usize, //this is how I will have it automatically assign an id to each body.
}

impl WorldSpace {
    //creating a new worldspace from defaults. Eventually I'm going to be able to save and load worldspaces from a file
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

    //updating positions of each body from it's velocity values
    fn update_positions(&mut self, sender: Sender<SimulationEvent>, dt: f32) {
        self.bodies
            .par_iter_mut()
            .enumerate() //getting the index of each element
            .for_each_with(sender, |sen, (idx, body)| {
                //cloning the sender so that it can be sent to each thread to send to the graphical/main thread
                sen.send(SimulationEvent::Move {
                    id: body.id.unwrap(), //the only time a body has no id is when it is first created.
                    idx,
                    pos: Vector2f::new(body.x, body.y),
                    change: Vector2f::new(body.xv * dt, body.yv * dt),
                })
                .unwrap();
                body.x += body.xv * dt; //i'm not entirely sure how to proceed on the delta time in calculations front, but i'll figure it out eventually
                body.y += body.yv * dt;
            });
    }

    //updating the velocities of the bodies from their accelerations
    fn update_velocity(&mut self, dt: f32) {
        self.bodies.par_iter_mut().for_each(|body| {
            body.xv += body.ax * dt;
            body.yv += body.ay * dt;
        });
    }

    //clearing the worldspace of bodies. I'm not sure whether I like handing in the sender, but i'll keep it for now
    pub fn clear(&mut self, sender: &mut Sender<SimulationEvent>) {
        self.bodies = Vec::new();
        sender.send(SimulationEvent::Clear).unwrap();
    }

    //the most important function for the entire simulation. this calculates the gravitational pull on each body
    fn update_accelerations(&mut self) {
        let g = self.g; //the force of gravity
        let softening = self.softening;
        for idx in 0..self.bodies.len() {
            let ax = Arc::new(Mutex::new(0.0)); //thread safety woot woot
            let ay = Arc::new(Mutex::new(0.0));
            let id = self.bodies[idx].id;
            let body_x = self.bodies[idx].x;
            let body_y = self.bodies[idx].y; //these are so that it's not double-borrowed and the compiler will stop yelling at me D:
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

    //freezing the simulation
    #[allow(unused)]
    pub fn stop(&mut self) {
        self.stopped = true;
    }

    //unfreezing the simulation :O
    #[allow(unused)]
    pub fn unstop(&mut self) {
        self.stopped = false;
    }

    //wrapping all of the internal functions together so that only one method has to be called to update and advance
    //hence the name
    pub fn update_advance(&mut self, dt: f32, sender: Sender<SimulationEvent>) {
        if !self.stopped {
            //so time can be "frozen"
            self.update_accelerations();
            self.update_velocity(dt);
            self.update_positions(sender, dt);
        }
    }

    //adding a body to the simulation
    pub fn add_body(&mut self, mut body: SpaceBody, sender: &mut Sender<SimulationEvent>) {
        //it should not have an id, because having an id pre-set is likely to cause problems
        //this way we know that no body will have had this id, because the prepared id can only go up!
        if body.id.is_none() {
            body.id = Some(self.prepared_id);
            self.prepared_id += 1;
        }

        //just sending the new body over to the graphics thread to be added in there
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

    //serialize the worldspace in a file
    pub fn to_file(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        let string = serde_json::to_string(&self)?;
        let mut file = fs::File::create(name)?;
        file.write_all(string.as_bytes())?;
        Ok(())
    }

    //deserialising the worldspace from the file
    pub fn from_file(name: String) -> Result<WorldSpace, Box<dyn std::error::Error>> {
        let unser = fs::read_to_string(name)?;
        let space = serde_json::from_str(&unser)?;
        Ok(space)
    }
}
