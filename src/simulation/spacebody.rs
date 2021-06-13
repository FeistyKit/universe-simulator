#[derive(Debug, StructOfArray)]
#[soa_derive = "Debug"]
pub struct Spacebody {
    pub x: f32,
    pub y: f32,
    pub mass: f32,
    pub colour: (u8, u8, u8),
    pub ax: f32,
    pub ay: f32,
    pub xv: f32,
    pub yv: f32,
    pub id: usize,
}

impl Spacebody {
    #[allow(clippy::clippy::too_many_arguments)]
    pub fn new(
        x: f32,
        y: f32,
        mass: f32,
        colour: Option<(u8, u8, u8)>,
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
            colour: colour.unwrap_or((255, 255, 255)),
            ax,
            ay,
            xv,
            yv,
            id,
        }
    }
}
