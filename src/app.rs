
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::{PistonWindow};

use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::glyph_cache::{GlyphCache};

use graphics::{clear, ellipse, rectangle};
use graphics::line::Line;
use graphics::text::Text;
use graphics::Transformed;

use image::RgbImage;

use std::path::{Path};
use std::f64::consts::{PI};

use super::visual::Starplot;
use super::dim::{Dim};
use super::consts::*;
use super::colors::*;

/// Action that is possible on the application
pub enum Action {
    SaveAsPhoto,
    Quit
}

/// App defines the global application variables for the visualization an Starplot
pub struct App {
    _img: RgbImage, // RGBA image
    gl: GlGraphics, // OpenGL drawing backend
    star: Starplot  // Starplot
}

////////////////////////////////////////////////////////////////////////////////
// Inherent methods
////////////////////////////////////////////////////////////////////////////////

impl App {

    /// Creates a new App instance
    pub fn new(img: RgbImage, gl: GlGraphics) -> App {
        App {_img: img,gl: gl, star: Starplot::new() }
    }

    /// Defines the Starplot configuration for the visualization
    pub fn def_star(&mut self, star: Starplot) {
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

    /// Get end point of dimension depending on it's value and angle
    fn get_end_point(initial: &[f64; 2], margin: &f64, angle: &f64, val: &f64) -> [f64; 2] {
        [ (initial[0] - margin)*val*angle.cos() + initial[0], 
          -(initial[1] - margin)*val*angle.sin() + initial[1] ]
    }

    fn get_label_point(initial: &[f64; 2], margin: &f64, margin_label: &f64, angle: &f64, label_size: usize, val: &f64) -> [f64; 2] {
        let mut extra_x: f64 = 0.0;
        if angle > &(PI*0.6) && angle < &(3.0*PI/2.0) {
            extra_x = 6.25*(label_size as f64);
        }

        [ (initial[0] - margin + margin_label)*val*angle.cos() + initial[0] - extra_x, 
          -(initial[1] - margin + margin_label)*val*angle.sin() + initial[1] ]
    }

    /// Get angle in radiants for each dimension
    fn get_angle(degree_div: &f64, div: f64) -> f64 {
        let degree: f64 = degree_div*(div + 1.0);
        (PI)*degree/180f64
    }

    /// Do the preprocessing part (calculate angles, contours)
    pub fn preproc(&mut self) {
        // get the degree divison for the number of dimensions
        let degree_div: f64 = 360.0/(self.star.dimensions.len() as f64);

        // get for each dimension it's final point (initial point is the center of the ellipse)
        for (i, dim) in self.star.dimensions.iter_mut().enumerate() {            
            let angle: f64 = App::get_angle(&degree_div, i as f64);  

            dim.i_point = [ CENTER[0], CENTER[1] ];
            dim.f_point = App::get_end_point( &[ CENTER[0], CENTER[1] ], 
                                              &MARGIN, 
                                              &angle, 
                                              &dim.val);

            dim.label.pos = App::get_label_point( &[ CENTER[0], CENTER[1] ], 
                                                  &MARGIN, 
                                                  &MARGIN_LABEL, 
                                                  &angle, 
                                                  dim.label.description.len(), 
                                                  &dim.val); 
        }

        // get the contour connections between dimensions
        self.star.contours = App::get_contour(&self.star.dimensions);
    }

    /// Render the Starplot
    pub fn render(&mut self, args: &RenderArgs, font_path: &Path) {  
        // define the position and size of the ellipse using a square
        let square = rectangle::square(self.star.x, self.star.y, self.star.size);

        // clone Starplot for avoiding borrow error
        let star: Starplot = self.star.clone();

        // get the CharacterCache that describes the used font properties
        let mut glyph = GlyphCache::new(font_path).unwrap();
        
        self.gl.draw(args.viewport(), |c, gl| {
            // clear the window
            clear(WHITE, gl);                 
            
            // draw dimensions and labels
            for dim in star.dimensions.iter() {
                Line::new(dim.color, 1.0).draw([dim.i_point[0], dim.i_point[1], dim.f_point[0], dim.f_point[1]], &c.draw_state, c.transform, gl);

                let transform = c.transform.trans(dim.label.pos[0], dim.label.pos[1]); 
                Text::new_color(dim.color, 10).draw(dim.label.description, &mut glyph, &c.draw_state, transform, gl);                
            }
            // draw contours
            for contour in star.contours.iter() {
                Line::new(BLACK, 1.0).draw(*contour, &c.draw_state, c.transform, gl);
            }

            // draw ellipse
            ellipse(BLACK, square, c.transform, gl);    
        });
    }

    /// Save visualization as a photo
    fn _photo(&mut self) {
        unimplemented!()
    }

    /// Handles the user input
    pub fn on_input(&mut self, inp: Input) -> Option<Action> {
        match inp {
            Input::Release(but) => {
                match but {
                    Button::Keyboard(Key::Q) => {
                        println!("'Q' pressed: exiting...");
                        return Some(Action::Quit);
                    }
                    Button::Keyboard(Key::Escape) => {
                        println!("'ESC' pressed: exiting...");
                        return Some(Action::Quit);
                    }
                    Button::Keyboard(Key::S) => {
                        return Some(Action::SaveAsPhoto);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        None
    }
}