use sfml::system::Vector2f;
#[allow(unused)]
#[derive(Debug)]
enum SimulationEvent {
    //events that can be sent to the main thread for graphics processing
    Move {
        id: usize,
        idx: usize,
        pos: Vector2f,
        change: Vector2f,
    },
    Delete {
        id: usize,
        idx: usize,
    },
    Add {
        id: usize,
        pos: Vector2f,
        color: (u8, u8, u8),
    },
}
