
use graphics::types::Color;

use super::consts::*;
use super::colors::BLACK;

/// Starplot defines the position, size and the different dimensions of
/// the visualization
#[derive(Clone)]
pub struct Starplot {
    pub size: f64,
    pub x: f64,
    pub y: f64,
    pub color: Color,
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
                   color: BLACK,
                   dimensions: Vec::new(),
                   contours: Vec::new() }
    }

    /// Initializes a new Starplot with a defined position and size
    pub fn init(size: f64, x: f64, y: f64) -> Starplot {
        Starplot { size: size, 
                   x: x, 
                   y: y, 
                   color: BLACK,
                   dimensions: Vec::new(),
                   contours: Vec::new() }
    }

    fn concat_label(val: &f64, range: &[f64; 2], label: &'static str) -> String {
        let range_str_a: &str = &*range[0].to_string();
        let range_str_b: &str = &*range[1].to_string();
        let val_str: &str = &*val.to_string();

        let mut label_string = String::with_capacity(label.len() + 
                                                     range_str_a.len() + 
                                                     range_str_b.len() +
                                                     val_str.len() + 
                                                     7 );
        
        label_string.push_str(label);
        label_string.push_str("(");
        label_string.push_str(val_str);
        label_string.push_str(") ");
        label_string.push_str("[");
        label_string.push_str(range_str_a);
        label_string.push_str(", ");
        label_string.push_str(range_str_b);
        label_string.push_str("]");

        label_string
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

        let label_string: String = Starplot::concat_label(&val, &range, label);
        
        self.dimensions.push(Dim { val: val, 
                                   range: range, 
                                   label: Label { description: label_string, pos: [0.0, 0.0] },
                                   color: color, 
                                   i_point: [0.0, 0.0], 
                                   f_point: [0.0, 0.0] });
    }
}

/// Dim defines a dimension (axis) of an Starplot
#[derive(Clone)]
pub struct Dim {
    pub val: f64,    
    pub range: [f64; 2],
    pub label: Label,
    pub color: [f32; 4],
    pub i_point: [f64; 2],
    pub f_point: [f64; 2]
}

/// Label defines the label description and its 
/// position in a Starplot
#[derive(Clone)]
pub struct Label {
    pub description: String,
    pub pos: [f64; 2]
}