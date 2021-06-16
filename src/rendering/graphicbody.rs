use sfml::graphics::CircleShape;
#[derive(Debug)]
pub struct GraphicBody<'shape> {
    pub id: usize,
    pub shape: CircleShape<'shape>,
}
