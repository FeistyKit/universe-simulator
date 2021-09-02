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
#[path = "./gui/widgets.rs"]
mod widgets;
#[path = "./simulation/worldspace.rs"]
mod worldspace;
use std::{sync::mpsc::channel, thread};

use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::Style,
};

use crate::{
    eventhandling::EventHandler, graphichandler::GraphicHandler, gui::gui_thread,
    simulation_thread::simulation_thread_start,
};

fn main() {
    //prepare the window to be used and set the frame limit for it
    let mut window = RenderWindow::new(
        (1000, 1000),
        "Universe Sim",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(45);

    //create the sender and reciever for communication with the simulation thread
    let (simulation_sender, simulation_receiver) = channel();

    //preparing the event handler. it will take events from sfml and send whatever is needed to the graphics thread or GUI thread
    let (mut handler, main_to_gui_reciever) = EventHandler::prepare();

    //preparing the channels between the GUI thread and the simulation thread
    let (gui_to_sim_sender, gui_to_sim_receiver) = channel();
    let (gui_to_main_sender, gui_to_main_reciever) = channel();

    //Making the simulation thread
    let simulation_thread = thread::Builder::new()
        .name("Simulation".to_string())
        .spawn(|| simulation_thread_start(simulation_sender, gui_to_sim_receiver))
        .unwrap(); //If making the simulation thread fails the whole program should end;
                   //it is a necesary part of the program.

    //making the GUI thread
    let gui_thread = thread::Builder::new()
        .name("GUI".to_string())
        .spawn(|| gui_thread(gui_to_main_sender, gui_to_sim_sender, main_to_gui_reciever))
        .unwrap();

    //prepare the graphics handler to handle the graphics on the main thread.
    //My goal is to have as few operations that are not directly graphics related as possible running on this thread
    let mut graphics_handler = GraphicHandler::new(simulation_receiver, gui_to_main_reciever);

    //the main loop of the graphics side of the program
    while window.is_open() {
        //using up all of the events in between the previous cycles
        while let Some(event) = window.poll_event() {
            //from here, the handler can take the event and do whatever is necesary with it
            handler.handle_events(event, &mut window);
        }

        window.clear(Color::BLACK);
        window.set_active(true);

        //receive everything from the other threads and apply it to the current graphical model
        graphics_handler.update();

        //render all of the changes to the window
        graphics_handler.draw(&mut window);
        window.display();
    }

    //ending the program
    simulation_thread.join().unwrap();
    gui_thread.join().unwrap();
}
