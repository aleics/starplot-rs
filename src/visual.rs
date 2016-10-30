
use graphics::types::Color;

use super::consts::*;
use super::colors::BLACK_BACKGROUND;

/// Starplot defines the position, size and the different dimensions of
/// the visualization
#[derive(Clone)]
pub struct Starplot {
    pub size_sphere: f64,
    pub size_ext: f64,
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
        Starplot { size_sphere: STARPLOT_SPHERE_SIZE,
                   size_ext: STARPLOT_SIZE, 
                   x: 0.0, 
                   y: 0.0, 
                   color: BLACK_BACKGROUND,
                   dimensions: Vec::new(),
                   contours: Vec::new() }
    }

    /// Initializes a new Starplot with a defined position and size
    pub fn init(size_sphere: f64, size: f64, x: f64, y: f64) -> Starplot {
        Starplot { size_sphere: size_sphere,
                   size_ext: size, 
                   x: x, 
                   y: y, 
                   color: BLACK_BACKGROUND,
                   dimensions: Vec::new(),
                   contours: Vec::new() }
    }

    /// Gets the full label description for the legend reference
    fn concat_label(val: &f64, range: &[f64; 2], label: &'static str, index: usize) -> String {
        let range_str_a: &str = &*range[0].to_string();
        let range_str_b: &str = &*range[1].to_string();
        let val_str: &str = &*val.to_string();

        let mut label_string = String::with_capacity(label.len() + 
                                                     range_str_a.len() + 
                                                     range_str_b.len() +
                                                     val_str.len() + 
                                                     7 );
        
        label_string.push_str(&*index.to_string());
        label_string.push_str(" ");
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

        let label_string: String = Starplot::concat_label(&val, &range, label, self.dimensions.len());

        let val = (val - range[0])/(range[1] - range[0]);
        
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

/// Legend defines the legend configuration for
/// the visualization
#[derive(Clone)]
pub struct Legend {
    pub description: Vec<String>,
    pub pos: Vec<[f64; 2]>
}

////////////////////////////////////////////////////////////////////////////////
// Inherent methods
////////////////////////////////////////////////////////////////////////////////

impl Legend {

    /// Creates a new Legend instance
    pub fn new() -> Legend {
        Legend {
            description: Vec::new(),
            pos: Vec::new()
        }
    }

    /// Add the description to the legend
    pub fn add_description(&mut self, description: String) {
        let mut pos: [f64; 2] = [0.0, 0.0];
        pos[1] = LEGEND_NEWLINE_STEP*((self.pos.len()+1) as f64);

        self.description.push(description);
        self.pos.push(pos);
    }
}