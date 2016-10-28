
use super::dim::{Dim, Label};
use super::consts::*;

/// Starplot defines the position, size and the different dimensions of
/// the visualization
#[derive(Clone)]
pub struct Starplot {
    pub size: f64,
    pub x: f64,
    pub y: f64,
    pub dimensions: Vec<Dim>,
    pub contours: Vec<[f64; 4]>
}

////////////////////////////////////////////////////////////////////////////////
// Inherent methods
////////////////////////////////////////////////////////////////////////////////

impl Starplot {

    /// Creates a new Starplot variable
    pub fn new() -> Starplot {
        Starplot { size: STARPLOT_SIZE, 
                   x: 0.0, 
                   y: 0.0, 
                   dimensions: Vec::new(),
                   contours: Vec::new() }
    }

    /// Initializes a new Starplot with a defined position and size
    pub fn init(size: f64, x: f64, y: f64) -> Starplot {
        Starplot { size: size, 
                   x: x, 
                   y: y, 
                   dimensions: Vec::new(),
                   contours: Vec::new() }
    }

    /// Adds a dimension to the Starplot with the defined configuration variables
    pub fn add_dim(&mut self, val: f64, range: [f64; 2], label: &'static str, color: [f32; 4]) {
        // if value is out of range: panic message
        if val < range[0] || val > range[1] { 
            panic!("value {val} out of range [{range_x}, {range_y}]", 
                   val = val, 
                   range_x = range[0], 
                   range_y = range[1]);
        }

        let val = (val - range[0])/(range[1] - range[0]);
        self.dimensions.push(Dim { val: val, 
                                   range: range, 
                                   label: Label { description: label, pos: [0.0, 0.0] },
                                   color: color, 
                                   i_point: [0.0, 0.0], 
                                   f_point: [0.0, 0.0] });
    }
}