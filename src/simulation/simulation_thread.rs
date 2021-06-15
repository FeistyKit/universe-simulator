use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
    time::Duration,
};

use crate::{
    spacebody::SpaceBody,
    transmission::{InputEvent, SimulationEvent},
    worldspace::WorldSpace,
};
const TIME_STEP: u32 = 1_000_000_000 / 45;
pub fn simulation_thread_start(
    mut sender: Sender<SimulationEvent>,
    receiver: Receiver<InputEvent>,
) {
    let mut space = WorldSpace::new();
    let body_one = SpaceBody::new(100.0, 100.0, 30.0, None, -10.0, 0.0, None);
    let body_two = SpaceBody::new(600.0, 600.0, 30.0, None, 10.0, 0.0, None);
    space.add_body(body_one, &mut sender);
    space.add_body(body_two, &mut sender);
    loop {
        while let Ok(event) = receiver.try_recv() {
            if let InputEvent::ShutDown = event {
                return;
            }
        }
        space.update_advance(10.0, sender.clone());
        thread::sleep(Duration::new(0, TIME_STEP));
    }
} //eventually going to be the main function for the simulation thread
