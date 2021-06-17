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
    let body_one = SpaceBody::new(100.0, 100.0, 30.0, 30.0, None, -1.0, 0.0, None);
    let body_two = SpaceBody::new(600.0, 600.0, 30.0, 30.0, None, 1.0, 0.0, None);
    space.add_body(body_one, &mut sender);
    space.add_body(body_two, &mut sender);
    'simulation: loop {
        if !cfg!(debug_assertions) {
            panic!("gui threading!");
        }
        while let Ok(event) = receiver.try_recv() {
            match event {
                InputEvent::LeftClick {
                    screen_pos: _,
                    pos,
                    highlighted_colour,
                    highlighted_size,
                    highlighted_mass,
                } => {
                    //TODO remove this this is just testing
                    let body = SpaceBody::new(
                        pos.x,
                        pos.y,
                        highlighted_mass,
                        highlighted_size,
                        Some((
                            highlighted_colour.r,
                            highlighted_colour.g,
                            highlighted_colour.b,
                        )),
                        0.0,
                        0.0,
                        None,
                    );
                    space.add_body(body, &mut sender);
                }
                InputEvent::ShutDown => break 'simulation,
            }
        }
        space.update_advance(10.0, sender.clone());
        thread::sleep(Duration::new(0, TIME_STEP));
    }
}
