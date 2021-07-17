use std::sync::mpsc::{channel, Receiver, Sender};

use sfml::{
    graphics::{RenderTarget, RenderWindow},
    system::Vector2,
    window::{mouse::Button, Event, Key},
};

use crate::transmission::InputEvent;

//the event handling struct for the main thread. It'll find what needs to be sent where and send it to be used later
pub struct EventHandler {
    pub trans_to_simulation: Sender<InputEvent>, //I definitely didn't name them this way just to say "trans" nope nope nope >.>
    pub trans_to_gui: Sender<InputEvent>,
}

impl EventHandler {
    //the main event-handling function. it is what events get passed into from the main function
    pub fn handle_events(&mut self, event: Event, window: &mut RenderWindow) {
        match event {
            Event::Closed => {
                //make sure the simulation thread shuts down with the rest of the program
                self.trans_to_simulation.send(InputEvent::ShutDown).unwrap();
                window.close();
            }
            Event::MouseButtonPressed { button, x, y } => {
                //Right now I'm only registering left clicks for input
                //I'm thinking of bringing up some graphics stuff with right click, but I have no solid ideas
                if button == Button::Left {
                    self.handle_left_button(x, y, window);
                }
            }
            Event::KeyPressed {
                code,
                alt,
                ctrl,
                shift,
                system,
            } => {
                //I probably won't use most of these modifiers, but eh
                self.handle_key_pressed(window, (code, alt, ctrl, shift, system));
            }
            //I'm not sure how many other events I might need, but I might implement a "pause on lost focus" feature, i'm not sure
            _ => {}
        }
    }
    fn handle_left_button(&mut self, x: i32, y: i32, window: &RenderWindow) {
        let pos = window.map_pixel_to_coords_current_view(Vector2::new(x, y)); //get real coordinates from the input

        self.trans_to_gui
            .send(InputEvent::LeftClick {
                screen_pos: Vector2::new(x, y),
                pos,
            })
            .unwrap(); //send coordinates to gui thread for processing
                       //TODO Put in gui things so that this does not go straight to the simulation
    }

    //preparing an event handler to be used by the program
    pub fn prepare() -> (EventHandler, Receiver<InputEvent>) {
        //prepares to start the program and the other threads

        //prepare transmittors to the simulation thread
        let (simul_tx, simul_receiver) = channel(); //simul_receiver is the reciever that is to be sent to the simulation thread

        //this is not being used as of right now, but it is necesary to create the struct
        let (gui_tx, _) = channel();

        let handler = EventHandler {
            trans_to_simulation: simul_tx,
            trans_to_gui: gui_tx,
        };

        (handler, simul_receiver)
    }

    fn handle_key_pressed(
        &mut self,
        window: &mut RenderWindow,
        details: (Key, bool, bool, bool, bool), //I *really* should not use all of the key details, it's probably going to get very confusing. But whatever
                                                //code, alt, ctrl, shift, system
    ) {
        match details.0 {
            Key::Up => {
                let mut view = window.view().to_owned(); //this operation is not expensive at all, it's just copying some floats
                view.zoom(0.75); //zoom in
                window.set_view(&view);
            }
            Key::Down => {
                let mut view = window.view().to_owned(); //again nothing to worry about, performance-wise
                view.zoom(1.25); //zoom out
                window.set_view(&view);
            }
            Key::C => {
                self.trans_to_simulation.send(InputEvent::Clear).unwrap();
            }
            _ => {}
        }
    }
}
