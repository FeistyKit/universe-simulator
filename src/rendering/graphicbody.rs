use sfml::graphics::CircleShape;

//wow I expected more to happen in this file *facepalms*
//The struct that allows for the syncing to the simulation thread via the id system
#[derive(Debug)]
pub struct GraphicBody<'shape> {
    pub id: usize,
    pub shape: CircleShape<'shape>,
}
