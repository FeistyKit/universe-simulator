use std::{
    fmt::Debug,
    sync::mpsc::{Receiver, Sender},
};

use sfml::{graphics::FloatRect, system::Vector2};

use crate::transmission::{GuiToGraphicsEvent, GuiToSimEvent, InputEvent};

//the amount by which the velocity is divided
const VEL_DIV: i32 = 10;

//the colour of the guideline
const GUIDELINE_COLOUR: (u8, u8, u8) = (255, 255, 255);

#[allow(unused)]
pub fn gui_thread(
    graphics_sender: Sender<GuiToGraphicsEvent>,
    sim_sender: Sender<GuiToSimEvent>,
    input_reciever: Receiver<InputEvent>,
) {
    let mut handler = GuiHandler::from_senders(graphics_sender, sim_sender, input_reciever);
    handler.start_recv();
}

//The struct that handles inputs for the GUI thread
pub struct GuiHandler {
    items: Vec<Box<dyn GuiWidget>>, //Widgets will be added to the list as different states change.
    graphics_sender: Sender<GuiToGraphicsEvent>,
    sim_sender: Sender<GuiToSimEvent>,
    input_receiver: Receiver<InputEvent>,
    clicked_widget_idx: Option<usize>, //the index of the widget that has been clicked, for performance reasons
    //if no widget has been clicked, it is None
    clicked_pos: Option<(i32, i32, f32, f32, i32, i32)>,
    //screen_coords, real_coords, moved
    //It's so that you can click twice to use
    //velocity on the body that will be added.
    highlighted_colour: (u8, u8, u8), //the values to be put if a body is added to the simulation
    highlighted_mass: f32,
    highlighted_size: f32,
}

pub trait GuiWidget: Debug {
    //The function that will be called when the screen is clicked.
    //will return ClickRegistered if the click happens on this widget
    fn check_clicked(&self, spot: Vector2<i32>) -> ClickResponse;

    //get the widget's rectangle
    fn rect(&self) -> FloatRect;

    //the function for when the widget is actually clicked
    fn click(
        &mut self,
        sim_sender: &mut Sender<GuiToSimEvent>,
        graphics_sender: &mut Sender<GuiToGraphicsEvent>,
        details: (Vector2<i32>, Vector2<f32>),
    );

    //what will be called when the mouse is moved and this widget is selected
    fn mouse_moved(
        &mut self,
        sim_sender: Sender<GuiToSimEvent>,
        graphics_sender: Sender<GuiToGraphicsEvent>,
    );

    //when the mouse is lifted up this will be called
    fn unclick(
        &mut self,
        sim_sender: Sender<GuiToSimEvent>,
        graphics_sender: Sender<GuiToGraphicsEvent>,
    );
}

#[allow(unused)]
pub enum ClickResponse {
    ClickRegistered,
    ClickNotUsed,
}

impl GuiHandler {
    //the reason that I use the blocking recieve here is because the gui never does anything on it's own. it's only for handling user input
    pub fn start_recv(mut self) {
        while let Ok(event) = self.input_receiver.recv() {
            //Break the loop if the shutdown event is triggered.
            if self.handle_events(event) {
                break;
            }
        }
    }

    //at some point, I'm going to do all of the processing on this thread, I just haven't gotten around to it
    pub fn handle_events(&mut self, event: InputEvent) -> bool {
        match event {
            InputEvent::LeftClick { screen_pos, pos } => self.left_click((screen_pos, pos)),
            InputEvent::ShutDown => {
                self.send_shut_down();
                return true;
            }
            InputEvent::Clear => self.sim_sender.send(GuiToSimEvent::Clear).unwrap(),
        }
        false
    }

    //handle left click
    fn left_click(&mut self, details: (Vector2<i32>, Vector2<f32>)) {
        //details is (position on screen, real position)

        let (new_screen_x, new_screen_y) = (details.0.x, details.0.y);
        let (real_x, real_y) = (details.1.x, details.1.y);

        //checking through the vector to see if any widgets are being clicked.
        //if any are, don't put a body onto the space.
        for idx in 0..self.items.len() {
            if let ClickResponse::ClickRegistered = self.items[idx].check_clicked(details.0) {
                self.clicked_widget_idx = Some(idx);
                self.items[idx].click(&mut self.sim_sender, &mut self.graphics_sender, details);
                return;
            }
        }

        //check to see if the screen has been clicked yet
        match self.clicked_pos {
            Some((old_screen_x, old_screen_y, _real_x, _real_y, moved_x, moved_y)) => {
                //calculate velocity
                let x_vel = (new_screen_x - old_screen_x - moved_x) / VEL_DIV;
                let y_vel = (new_screen_y - old_screen_y - moved_y) / VEL_DIV;

                //if it has, add a body to the simulation with the specified parameters
                self.sim_sender
                    .send(GuiToSimEvent::AddBody {
                        color: self.highlighted_colour,
                        mass: self.highlighted_mass,
                        size: self.highlighted_size,
                        pos: details.1,
                        vel: (x_vel as f32, y_vel as f32),
                    })
                    .expect("Could not send to the Simulation thread!");

                //reset the clicked position
                self.clicked_pos = None;
            }
            None => {
                //If it has not, prepare to add a body
                self.clicked_pos = Some((new_screen_x, new_screen_y, real_x, real_y, 0, 0));

                //also tell the graphics thread to start the guideline
                self.graphics_sender
                    .send(GuiToGraphicsEvent::EnableGuideline {
                        position: (real_x, real_y),
                        colour: GUIDELINE_COLOUR,
                    })
                    .unwrap();
            }
        }
    }

    //sends the shutdown events to the other threads
    fn send_shut_down(&mut self) {
        self.graphics_sender
            .send(GuiToGraphicsEvent::ShutDown)
            .unwrap();
        self.sim_sender.send(GuiToSimEvent::Exit).unwrap();
    }

    //create a GuiHandler by passing in the senders to it. I may change these values later.
    pub fn from_senders(
        graphics_sender: Sender<GuiToGraphicsEvent>,
        sim_sender: Sender<GuiToSimEvent>,
        input_receiver: Receiver<InputEvent>,
    ) -> GuiHandler {
        GuiHandler {
            items: Vec::new(),
            graphics_sender,
            sim_sender,
            input_receiver,
            clicked_widget_idx: None,
            highlighted_colour: (255, 255, 255),
            highlighted_mass: 20.0,
            highlighted_size: 25.0,
            clicked_pos: None,
        }
    }
}
