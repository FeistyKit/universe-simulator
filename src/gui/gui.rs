use std::{
    fmt::Debug,
    sync::mpsc::{Receiver, Sender},
};

use sfml::{graphics::FloatRect, system::Vector2};

use crate::transmission::{GuiToGraphicsEvent, GuiToSimEvent, InputEvent};

#[allow(unused)]
pub fn gui_thread(handler: GuiHandler) {}

//The struct that handles inputs for the GUI thread
pub struct GuiHandler {
    items: Vec<Box<dyn GuiWidget>>,
    graphics_sender: Sender<GuiToGraphicsEvent>,
    sim_sender: Sender<GuiToSimEvent>,
    input_receiver: Receiver<InputEvent>,
    clicked_widget_idx: Option<usize>, //the index of the widget that has been clicked, for performance reasons
    //if no widget has been clicked, it is None
    highlighted_colour: (u8, u8, u8), //the values to be put if a body is added to the simulation
    highlighted_mass: f32,
    highlighted_size: f32,
}

trait GuiWidget: Debug {
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

enum ClickResponse {
    ClickRegistered,
    ClickNotUsed,
}

impl GuiHandler {
    //the reason that I use the blocking recieve here is because the gui never does anything on it's own. it's only for handling user input
    pub fn recv(&mut self) {
        if let Ok(event) = self.input_receiver.recv() {
            self.handle_events(event);
        }
    }

    pub fn handle_events(&mut self, event: InputEvent) {
        match event {
            InputEvent::LeftClick { screen_pos, pos } => todo!(),
            InputEvent::ShutDown => todo!(),
            InputEvent::Clear => todo!(),
        }
    }

    //handle left click
    fn left_click(&mut self, details: (Vector2<i32>, Vector2<f32>)) {
        //checking through the vec to see if any widgets are being clicked.
        //if any are, don't put a body onto the space.
        for idx in 0..self.items.len() {
            if let ClickResponse::ClickRegistered = self.items[idx].check_clicked(details.0) {
                self.clicked_widget_idx = Some(idx);
                self.items[idx].click(&mut self.sim_sender, &mut self.graphics_sender, details);
                return;
            }
        }

        //if not, add a body to the simulation with the specified parameters
        self.sim_sender
            .send(GuiToSimEvent::AddBody {
                color: self.highlighted_colour,
                mass: self.highlighted_mass,
                size: self.highlighted_size,
                pos: details.1,
            })
            .expect("Could not send to the Simulation thread!");
    }
}
