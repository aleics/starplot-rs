extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate opengl_graphics;
extern crate find_folder;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::{PistonWindow};
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::glyph_cache::{GlyphCache};
use graphics::*;
use std::path::{Path};

const WINDOW_SIZE: f64 = 400.0;
const CENTER: [f64; 2] = [WINDOW_SIZE/2.0, WINDOW_SIZE/2.0];
const MARGIN: f64 = 50.0;

const STARPLOT_SIZE: f64 = 20.0;
const STARPLOT_POS_X: f64 = 190.0;
const STARPLOT_POS_Y: f64 = 190.0;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK:   [f32; 4] = [0.0, 0.0, 0.0, 1.0];

#[derive(Clone)]
/// Starplot defines the position, size and the different dimensions of
/// the visualization
pub struct Starplot {
    pub size: f64,
    pub x: f64,
    pub y: f64,
    pub dimensions: Vec<Dim>
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
                   dimensions: Vec::new() }
    }

    /// Initializes a new Starplot with a defined position and size
    pub fn init(size: f64, x: f64, y: f64) -> Starplot {
        Starplot { size: size, 
                   x: x, 
                   y: y, 
                   dimensions: Vec::new() }
    }

    /// Adds a dimension to the Starplot with the defined configuration variables
    pub fn add_dim(&mut self, val: f64, range: [f64; 2], label: &'static str) {
        // if value is out of range: panic message
        if val < range[0] || val > range[1] { 
            panic!("value {val} out of range [{range_x}, {range_y}]", 
                   val = val, 
                   range_x = range[0], 
                   range_y = range[1]);
        }

        let val = (val - range[0])/(range[1] - range[0]);
        self.dimensions.push(Dim {val: val, range: range, label: label, i_point: [0.0, 0.0], f_point: [0.0, 0.0] });
    }
}

#[derive(Clone)]
/// Dim defines a dimension (axis) of an Starplot
pub struct Dim {
    val: f64,    
    range: [f64; 2],
    label: &'static str,
    i_point: [f64; 2],
    f_point: [f64; 2]
}

/// App defines the global application variables for the visualization an Starplot
pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    star: Starplot  // Starplot
}

////////////////////////////////////////////////////////////////////////////////
// Inherent methods
////////////////////////////////////////////////////////////////////////////////

impl App {

    /// Creates a new App instance
    pub fn new(gl: GlGraphics) -> App {
        App {gl: gl, star: Starplot::new() }
    }

    /// Defines the Starplot configuration for the visualization
    fn def_star(&mut self, star: Starplot) {
        self.star = star;
    }

    /// Get the contours of the Starplot dimensions
    fn get_contour(dim_vec: &Vec<Dim>) -> Vec<[f64; 4]> {
        let mut out: Vec<[f64; 4]> = Vec::new();

        for i in 0..dim_vec.len() {
            // if last dimension: connect with the first
            if (i + 1) == dim_vec.len() { 
                out.push([ dim_vec[i].f_point[0], dim_vec[i].f_point[1], 
                           dim_vec[0].f_point[0], dim_vec[0].f_point[1] ]);

                break;
            }
            // connect each dimension
            out.push([ dim_vec[i].f_point[0], dim_vec[i].f_point[1], 
                       dim_vec[i+1].f_point[0], dim_vec[i+1].f_point[1] ]);
        }

        out
    }

    // Get end point of dimension depending on it's value and angle
    fn get_end_point(initial: &[f64; 2], margin: &f64, angle: &f64, val: &f64) -> [f64; 2] {
        [ (initial[0] - margin)*val*angle.cos() + initial[0], 
          -(initial[1] - margin)*val*angle.sin() + initial[1] ]
    }

    // Get angle in radiants for each dimension
    fn get_angle(degree_div: &f64, div: f64) -> f64 {
        let degree: f64 = degree_div*(div + 1.0);
        (std::f64::consts::PI)*degree/180f64
    }

    // Render the Starplot
    fn render(&mut self, args: &RenderArgs, font_path: &Path) {  
        // define the position and size of the ellipse using a square
        let square = rectangle::square(self.star.x, self.star.y, self.star.size);

        let mut star: Starplot = self.star.clone();

        // get the degree divison for the number of dimensions
        let degree_div: f64 = 360.0/(star.dimensions.len() as f64);

        // get for each dimension it's final point (initial point is the center of the ellipse)
        for (i, dim) in star.dimensions.iter_mut().enumerate() {            
            let angle: f64 = App::get_angle(&degree_div, i as f64);  

            dim.i_point = [ CENTER[0], CENTER[1] ];
            dim.f_point = App::get_end_point(&[ CENTER[0], CENTER[1] ], &MARGIN, &angle, &dim.val); 
        }

        // get the contour connections between dimensions
        let contours: Vec<[f64; 4]> = App::get_contour(&star.dimensions);

        let mut glyph = GlyphCache::new(font_path).unwrap();
        
        self.gl.draw(args.viewport(), |c, gl| {
            // clear the window
            clear(WHITE, gl);

            // draw ellipse
            ellipse(BLACK, square, c.transform, gl);            

            text::Text::new_color(BLACK, 32).draw("hello world!", 
                                                      &mut glyph, 
                                                      &c.draw_state, 
                                                      c.transform, 
                                                      gl);            
            
            // draw dimensions
            for dim in star.dimensions.iter() {
                Line::new(BLACK, 1.0).draw([dim.i_point[0], dim.i_point[1], dim.f_point[0], dim.f_point[1]], &c.draw_state, c.transform, gl);                
            }
            // draw contours
            for contour in contours.iter() {
                Line::new(BLACK, 1.0).draw(*contour, &c.draw_state, c.transform, gl);
            }
        });
    }

    /*fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }*/
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Piston window
    let mut window: PistonWindow = WindowSettings::new(
            "starplot",
            [WINDOW_SIZE as u32, WINDOW_SIZE as u32]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Get the font for the text representation
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");

    // Generate an Starplot
    let mut starplot = Starplot::init(STARPLOT_SIZE, 
                                      STARPLOT_POS_X, 
                                      STARPLOT_POS_Y);

    // Add dimensions to the Starplot
    starplot.add_dim(0.2, [0.0, 1.0], "first");
    starplot.add_dim(0.8, [0.0, 1.0], "second");
    starplot.add_dim(50.0, [0.0, 200.0], "third");
    starplot.add_dim(0.4, [0.0, 1.0], "forth");
    starplot.add_dim(0.7, [0.0, 1.0], "fifth");
    starplot.add_dim(0.6, [0.0, 1.0], "sixth");
    starplot.add_dim(0.5, [0.0, 1.0], "seventh");

    // Add Starplot to the Application
    let mut app = App::new(GlGraphics::new(opengl));
    app.def_star(starplot);

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {            
            app.render(&r, &font.as_path()); // render the Application
        }

        /*if let Some(_) = e.update_args() {
        }*/
    }
}