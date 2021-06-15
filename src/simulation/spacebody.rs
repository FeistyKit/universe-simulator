use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)] //idk why but for some reason this is killing rust analyzer
pub struct SpaceBody {
    pub x: f32,
    pub y: f32,
    pub mass: f32,
    pub colour: (u8, u8, u8),
    pub ax: f32,
    pub ay: f32,
    pub xv: f32,           //x velocity
    pub yv: f32,           //y velocity
    pub id: Option<usize>, //thinking of using an "id" system to verify bodies more effectively between the threads. It will only be "None" when first initialized
}

impl SpaceBody {
    #[allow(clippy::too_many_arguments, unused)]
    pub fn new(
        x: f32,
        y: f32,
        mass: f32,
        colour: Option<(u8, u8, u8)>,
        xv: f32,
        yv: f32,
        id: Option<usize>,
    ) -> SpaceBody {
        //creating a new spacebody. these arguments will be handled automatically.
        SpaceBody {
            x,
            y,
            mass,
            colour: colour.unwrap_or((255, 255, 255)),
            ax: 0.0, //acceleration will be applied in the worldspace
            ay: 0.0,
            xv,
            yv,
            id,
        }
    }
}
