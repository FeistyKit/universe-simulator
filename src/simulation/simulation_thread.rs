use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
    time::{Duration, Instant},
};

use crate::{
    spacebody::SpaceBody,
    transmission::{GuiToSimEvent, SimulationEvent},
    worldspace::WorldSpace,
};

//the amount of nanoseconds that are in 1/45th of a second
const TIME_STEP: u32 = 1_000_000_000 / 45;

//start the simulation thread
pub fn simulation_thread_start(
    mut sender: Sender<SimulationEvent>,
    receiver: Receiver<GuiToSimEvent>,
) {
    //prepare the worldspace by loading defaults
    let mut space = WorldSpace::new();

    //add some default bodies in so that it's not a bland black screen on startup
    let body_one = SpaceBody::new(100.0, 100.0, 30.0, 30.0, None, -1.0, 0.0, None);
    let body_two = SpaceBody::new(600.0, 600.0, 30.0, 30.0, None, 1.0, 0.0, None);
    space.add_body(body_one, &mut sender);
    space.add_body(body_two, &mut sender);

    //preparations for my terrible delta time function
    let mut time = Instant::now();
    'simulation: loop {
        while let Ok(event) = receiver.try_recv() {
            //handle events received from the gui thread
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
    event: GuiToSimEvent,
    space: &mut WorldSpace,
    sender: &mut Sender<SimulationEvent>,
) -> bool {
    match event {
        GuiToSimEvent::AddBody {
            color,
            size,
            mass,
            pos,
        } => {
            //adding the body to the worldspace. the "add_body" method will handle sending it to the main thread
            let body = SpaceBody::new(
                pos.x,
                pos.y,
                mass,
                size,
                Some((color.0, color.1, color.2)),
                0.0,
                0.0,
                None,
            );
            space.add_body(body, sender);
        }
        GuiToSimEvent::Exit => return true,
        GuiToSimEvent::Clear => {
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
