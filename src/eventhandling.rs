use std::sync::mpsc::{channel, Sender};

use sfml::{
    graphics::{Color, RenderTarget, RenderWindow, View},
    system::{Vector2, Vector2f},
    window::{mouse::Button, Event},
};

use crate::transmission::InputEvent;

pub struct EventHandler {
    //The event handling struct for the main function
    pub trans_to_simulation: Sender<InputEvent>,
    pub trans_to_gui: Sender<InputEvent>,
}
impl EventHandler {
    pub fn handle_events(&mut self, event: Event, window: &mut RenderWindow) {
        match event {
            Event::Closed => {
                window.close();
            }
            Event::MouseButtonPressed { button, x, y } => {
                if button == Button::Left {
                    self.handle_left_button(x, y, window);
                }
            }
            _ => {
                println!("{:?}", event)
            }
        }
    }
    fn handle_left_button(&mut self, x: i32, y: i32, window: &RenderWindow) {
        let pos = window.map_pixel_to_coords_current_view(Vector2::new(x, y));
        self.trans_to_simulation
            .send(InputEvent::LeftClickOnSpace {
                pos,
                highlighted_colour: Color::WHITE,
                highlighted_size: 30.0,
                highlighted_mass: 25.0,
            })
            .unwrap();
    }
    pub fn prepare(window: &mut RenderWindow) -> EventHandler {
        //prepares to start the program and the other threads
        let (simul_tx, _) = channel();
        let (gui_tx, _) = channel();
        let handler = EventHandler {
            trans_to_simulation: simul_tx,
            trans_to_gui: gui_tx,
        };
        let size = window.size();

        let view = View::new(
            Vector2f::new(0.0, 0.0) + (Vector2f::new(size.x as f32 / 2.0, size.y as f32 / 2.0)),
            Vector2f::new(size.x as f32, size.y as f32),
        );
        window.set_view(&view);
        handler
    }
}
