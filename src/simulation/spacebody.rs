use sfml::graphics::Color;

#[derive(Debug, StructOfArray)]
#[soa_derive = "Debug"]
pub struct Spacebody {
    pub x: f32,
    pub y: f32,
    pub mass: f32,
    pub colour: Color,
    pub ax: f32,
    pub ay: f32,
    pub xv: f32,   //x velocity
    pub yv: f32,   //y velocity
    pub id: usize, //thinking of using an "id" system to verify bodies more effectively between the threads
}

impl Spacebody {
    #[allow(clippy::clippy::too_many_arguments)]
    pub fn new(
        x: f32,
        y: f32,
        mass: f32,
        colour: Option<Color>,
        ax: f32,
        ay: f32,
        xv: f32,
        yv: f32,
        id: usize,
    ) -> Spacebody {
        Spacebody {
            x,
            y,
            mass,
            colour: colour.unwrap_or_else(|| Color::rgb(255, 255, 255)),
            ax,
            ay,
            xv,
            yv,
            id,
        }
    }
}
