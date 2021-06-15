mod eventhandling;
#[path = "./gui/gui.rs"]
mod gui;
#[path = "./simulation/simulation_thread.rs"]
mod simulation_thread;
#[path = "./simulation/spacebody.rs"]
mod spacebody;
mod transmission;
#[path = "./simulation/worldspace.rs"]
mod worldspace;
use std::{
    sync::mpsc::channel,
    thread::{self, Thread},
};

use sfml::{graphics::RenderWindow, window::Style};

use crate::{eventhandling::EventHandler, simulation_thread::simulation_thread_start};

fn main() {
    let mut window = RenderWindow::new(
        (1000, 1000),
        "Universe Sim",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(45);
    let (simulation_sender, _) = channel();
    let (mut handler, simulation_receiver) = EventHandler::prepare(&mut window);
    let simulation_thread =
        thread::spawn(|| simulation_thread_start(simulation_sender, simulation_receiver));
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            handler.handle_events(event, &mut window)
        }
        window.set_active(true);
        window.display();
    }
    simulation_thread.join().unwrap();
}
