mod eventhandling;
#[path = "./simulation/simulation_thread.rs"]
mod simulation_thread;
#[path = "./simulation/spacebody.rs"]
mod spacebody;

mod transmission;
use sfml::window::{Style, Window};

use crate::eventhandling::handle_events;

#[macro_use]
extern crate soa_derive;

fn main() {
    let mut window = Window::new(
        (1000, 1000),
        "tester window",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(45);
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            handle_events(event, &mut window);
        }
        window.set_active(true);
        window.display();
    }
}
