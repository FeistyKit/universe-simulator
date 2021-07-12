use sfml::{
    graphics::Color,
    system::{Vector2, Vector2f},
};

//events that can be sent to the main thread for graphics processing
#[allow(unused)]
#[derive(Debug)]
pub enum SimulationEvent {
    //a body has moved
    Move {
        id: usize,
        idx: usize,
        pos: Vector2f,
        change: Vector2f,
    },
    //deleting a body from both vectors
    Delete {
        id: usize,
        idx: usize,
    },
    //adding a body to both vectors
    Add {
        id: usize,
        pos: Vector2f,
        color: (u8, u8, u8),
        size: f32,
    },
    //clearing both vectors
    Clear,
}

//Events from input to be sent to the simulation thread
//At some point it will be sent to the graphics thread, which will then send it to the simulation thread.
//These will only be sent by the graphics handler. I hope to make a new type of event
//that will be sent from the GUI thread to the simulation thread after the GUI thread processes inputs
#[derive(Debug)]
pub enum InputEvent {
    //a raw left click and every piece of information that I think might be necesary
    LeftClick {
        screen_pos: Vector2<i32>,
        pos: Vector2f,
        highlighted_colour: Color, //this hasn't been implemented yet, but I hope to set up a system with the gui
        highlighted_size: f32, //so that there is a little "example planet" in the gui that you can change different parts of:
        highlighted_mass: f32, //mass, radius, colour, etc. When a new planet gets added, it will get added with those stats.
    },

    //sending events to the other threads so it can be cleaned up properly
    ShutDown,

    //clearing the simulation and graphic vectors of bodies
    Clear,
}

//Events from the GUI thread to be sent to simulation thread
#[derive(Debug)]
pub enum GuiToSimEvent {}

//An event to be sent from the gui thread to the graphics thread.
#[derive(Debug)]
pub enum GuiToGraphicsEvent {}
