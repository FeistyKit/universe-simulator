use sfml::window::{Event, Window};

pub fn handle_events(event: Event, window: &mut Window) {
    if event == Event::Closed {
        window.close();
    } else {
        println!("{:?}", event);
    }
}
