use std::sync::mpsc::Sender;

use sfml::{graphics::FloatRect, system::Vector2};

use crate::{
    gui::{ClickResponse, GuiWidget},
    transmission::{GuiToGraphicsEvent, GuiToSimEvent},
};

//a placeholder widget for until I make other widgets to satisfy the compiler. It will never be constructed
#[derive(Debug)]
pub struct PlaceholderWidget;

#[allow(unused)]
//these methods will never be called, so they will stay at "todo"
impl GuiWidget for PlaceholderWidget {
    fn check_clicked(&self, spot: Vector2<i32>) -> ClickResponse {
        todo!()
    }

    fn rect(&self) -> FloatRect {
        todo!()
    }

    fn click(
        &mut self,
        sim_sender: &mut Sender<GuiToSimEvent>,
        graphics_sender: &mut Sender<GuiToGraphicsEvent>,
        details: (Vector2<i32>, Vector2<f32>),
    ) {
        todo!()
    }

    fn mouse_moved(
        &mut self,
        sim_sender: Sender<GuiToSimEvent>,
        graphics_sender: Sender<GuiToGraphicsEvent>,
    ) {
        todo!()
    }

    fn unclick(
        &mut self,
        sim_sender: Sender<GuiToSimEvent>,
        graphics_sender: Sender<GuiToGraphicsEvent>,
    ) {
        todo!()
    }
}
