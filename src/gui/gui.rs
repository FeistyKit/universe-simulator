use std::{fmt::Debug, sync::mpsc::Sender};

use sfml::graphics::FloatRect;

use crate::transmission::{GuiToGraphicsEvent, GuiToSimEvent};

#[allow(unused)]
pub fn gui_thread() {}

//The struct that handles inputs for the GUI thread
pub struct GuiHandler {
    items: Vec<FloatRect>,
}
trait GuiWidget: Debug {
    //The function that will be called when the screen is clicked.
    //will return ClickRegistered if the click happens on this widget
    fn check_clicked(&self, spot: (i32, i32)) -> ClickResponse;

    //get the widget's rectangle
    fn rect(&self) -> FloatRect;

    //the function for when the widget is actually clicked
    fn click(
        &mut self,
        sim_sender: Sender<GuiToSimEvent>,
        graphics_sender: Sender<GuiToGraphicsEvent>,
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
