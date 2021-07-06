use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
    time::{Duration, Instant},
};

use crate::{
    spacebody::SpaceBody,
    transmission::{InputEvent, SimulationEvent},
    worldspace::WorldSpace,
};

//the amount of nanoseconds that are in 1/45th of a second
const TIME_STEP: u32 = 1_000_000_000 / 45;

//start the simulation thread
pub fn simulation_thread_start(
    mut sender: Sender<SimulationEvent>,
    receiver: Receiver<InputEvent>,
) {
    //prepare the worldspace by loading defaults
    let mut space = WorldSpace::new();

    //add some default bodies in so that it's not a bland black screen on startup
    let body_one = SpaceBody::new(100.0, 100.0, 30.0, 30.0, None, -1.0, 0.0, None);
    let body_two = SpaceBody::new(600.0, 600.0, 30.0, 30.0, None, 1.0, 0.0, None);
    space.add_body(body_one, &mut sender);
    space.add_body(body_two, &mut sender);

    //prepare for my terrible delta time function
    let mut time = Instant::now();
    'simulation: loop {
        //so I can't publish a release of the code without the GUI thread implemented
        if !cfg!(debug_assertions) {
            panic!("gui threading!");
        }
        while let Ok(event) = receiver.try_recv() {
            //handle events received from the main function
            if handle_received_events(event, &mut space, &mut sender) {
                break 'simulation;
            }
        }
        //Getting the elapsed time
        let elapsed = time.elapsed();

        //My attempt at implementing delta time. I don't think it's worked.
        space.update_advance(
            to_simulation_time(elapsed + Duration::new(0, TIME_STEP)),
            sender.clone(),
        );
        thread::sleep(Duration::new(0, TIME_STEP) - elapsed);
        time = Instant::now();
    }
}

fn handle_received_events(
    event: InputEvent,
    space: &mut WorldSpace,
    sender: &mut Sender<SimulationEvent>,
) -> bool {
    match event {
        InputEvent::LeftClick {
            screen_pos: _,
            pos,
            highlighted_colour,
            highlighted_size,
            highlighted_mass,
        } => {
            //I will eventually remove this when I implement the GUI thread. It's a stopgap measure.
            //Just adding a new body directly, instead of how it is supposed to be
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
            space.add_body(body, sender);
        }
        InputEvent::ShutDown => return true,
        InputEvent::Clear => {
            println!("clearing space!");
            space.clear(sender);
        }
    }
    false
}

//a function for converting a duration to a float to be used in my delta time thing
#[allow(unused)]
fn to_simulation_time(time_passed: Duration) -> f32 {
    let nanos = time_passed.subsec_nanos();
    (nanos / TIME_STEP) as f32 * 10.0
}
