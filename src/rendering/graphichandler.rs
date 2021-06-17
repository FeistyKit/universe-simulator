use std::sync::mpsc::Receiver;

use crate::{graphicbody::GraphicBody, transmission::SimulationEvent};

use sfml::graphics::{CircleShape, Color, RenderTarget, Shape, Transformable};

pub struct GraphicHandler<'bodies> {
    bodies: Vec<GraphicBody<'bodies>>,
    receiver: Receiver<SimulationEvent>,
}
impl<'bodies> GraphicHandler<'bodies> {
    pub fn new(receiver: Receiver<SimulationEvent>) -> GraphicHandler<'bodies> {
        GraphicHandler {
            bodies: Vec::new(),
            receiver,
        }
    }
    pub fn update(&mut self) {
        //the function to update the graphics handler.
        while let Ok(input) = self.receiver.try_recv() {
            //dealing with every input
            match input {
                SimulationEvent::Move {
                    id,
                    idx,
                    pos,
                    change,
                } => {
                    assert_eq!(id, self.bodies[idx].id); //making sure we haven't desynced from the simulation thread
                    debug_assert_eq!(self.bodies[idx].shape.position(), pos);
                    self.bodies[idx].shape.move_(change);
                }
                SimulationEvent::Delete { id, idx } => {
                    assert_eq!(self.bodies[idx].id, id);
                    self.bodies.remove(idx);
                }
                SimulationEvent::Add {
                    id,
                    pos,
                    color,
                    size,
                } => {
                    let mut circle = CircleShape::new(size, 25);
                    circle.set_fill_color(Color::rgb(color.0, color.1, color.2));
                    circle.set_position(pos);
                    let body = GraphicBody { id, shape: circle };
                    self.bodies.push(body);
                }
                SimulationEvent::Clear => self.bodies = Vec::new(),
            }
        }
    }
    pub fn draw(&self, target: &mut dyn RenderTarget) {
        for body in &self.bodies {
            target.draw(&body.shape);
        }
    }
}
