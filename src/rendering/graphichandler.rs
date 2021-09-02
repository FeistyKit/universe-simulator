use std::sync::mpsc::Receiver;

use crate::{
    graphicbody::GraphicBody,
    transmission::{GuiToGraphicsEvent, SimulationEvent},
};

use sfml::graphics::{CircleShape, Color, RenderTarget, Shape, Transformable};

#[allow(unused)]
//the graphics handler struct
pub struct GraphicHandler<'bodies> {
    bodies: Vec<GraphicBody<'bodies>>,
    sim_receiver: Receiver<SimulationEvent>,
    gui_reciever: Receiver<GuiToGraphicsEvent>, //recievers from the other threads
}

impl<'bodies> GraphicHandler<'bodies> {
    //a new graphics handler
    pub fn new(
        sim_receiver: Receiver<SimulationEvent>,
        gui_reciever: Receiver<GuiToGraphicsEvent>,
    ) -> GraphicHandler<'bodies> {
        GraphicHandler {
            bodies: Vec::new(),
            sim_receiver,
            gui_reciever,
        }
    }

    //the function to update the graphics handler.
    pub fn update(&mut self) {
        while let Ok(input) = self.sim_receiver.try_recv() {
            self.handle_sim_input(input);
        }

        while let Ok(input) = self.gui_reciever.try_recv() {
            self.handle_gui_input(input);
        }
    }

    //handle info from the simulation thread
    fn handle_sim_input(&mut self, input: SimulationEvent) {
        match input {
            //moving a body
            SimulationEvent::Move {
                id,
                idx,
                pos,
                change,
            } => {
                assert_eq!(id, self.bodies[idx].id); //making sure we haven't desynced from the simulation thread
                debug_assert_eq!(self.bodies[idx].shape.position(), pos); //the same, but only debug assert because I think it's fairly costly to do
                self.bodies[idx].shape.move_(change);
            }

            //deleting a body
            SimulationEvent::Delete { id, idx } => {
                assert_eq!(self.bodies[idx].id, id); //the id system is quite handy I think
                self.bodies.remove(idx);
            }

            //adding a body
            SimulationEvent::Add {
                id,
                pos,
                color,
                size,
            } => {
                //preparing a circle to be the main bit of the graphicbody
                let mut circle = CircleShape::new(size, 25); //We don't need too many points for the outside of the circles, so I chose 25
                circle.set_fill_color(Color::rgb(color.0, color.1, color.2));
                circle.set_position(pos);
                let body = GraphicBody { id, shape: circle };

                //actually adding the body to the list of bodies
                self.bodies.push(body);
            }

            //clearing the list of bodies
            SimulationEvent::Clear => self.bodies = Vec::new(),
        }
    }

    //Handle input from the Gui thread
    fn handle_gui_input(&mut self, input: GuiToGraphicsEvent) {
        match input {
            GuiToGraphicsEvent::ShutDown => {} //nothing to actually do on shutdown
        }
    }

    //drawing the stuff to the window
    pub fn draw(&self, target: &mut dyn RenderTarget) {
        for body in &self.bodies {
            target.draw(&body.shape);
        }
    }
}
