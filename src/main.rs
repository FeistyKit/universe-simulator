mod eventhandling;
#[path = "./simulation/simulation_thread.rs"]
mod simulation_thread;
#[path = "./simulation/spacebody.rs"]
mod spacebody;

#[path = "./gui/gui.rs"]
mod gui;
mod transmission;
use sfml::{graphics::RenderWindow, window::Style};

use crate::eventhandling::EventHandler;

#[macro_use]
extern crate soa_derive;

fn main() {
    let mut window = RenderWindow::new(
        (1000, 1000),
        "tester window",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(45);
    let mut handler = EventHandler::prepare(&mut window);
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            handler.handle_events(event, &mut window)
        }
        window.set_active(true);
        window.display();
    }
}
