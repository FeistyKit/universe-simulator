mod eventhandling;
#[path = "./rendering/graphicbody.rs"]
mod graphicbody;
#[path = "./rendering/graphichandler.rs"]
mod graphichandler;
#[path = "./gui/gui.rs"]
mod gui;
#[path = "./simulation/simulation_thread.rs"]
mod simulation_thread;
#[path = "./simulation/spacebody.rs"]
mod spacebody;
mod transmission;
#[path = "./simulation/worldspace.rs"]
mod worldspace;
use std::{sync::mpsc::channel, thread};

use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::Style,
};

use crate::{
    eventhandling::EventHandler, graphichandler::GraphicHandler,
    simulation_thread::simulation_thread_start,
};

fn main() {
    let mut window = RenderWindow::new(
        (1000, 1000),
        "Universe Sim",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(45);
    let (simulation_sender, simulation_receiver) = channel();
    let mut graphics_handler = GraphicHandler::new(simulation_receiver);
    let (mut handler, simulation_receiver) = EventHandler::prepare(&mut window);
    let simulation_thread = thread::Builder::new()
        .name("Simulation".to_string())
        .spawn(|| simulation_thread_start(simulation_sender, simulation_receiver))
        .unwrap();
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            handler.handle_events(event, &mut window);
        }
        window.clear(Color::BLACK);
        window.set_active(true);
        graphics_handler.update();
        graphics_handler.draw(&mut window);
        window.display();
    }
    simulation_thread.join().unwrap();
}
