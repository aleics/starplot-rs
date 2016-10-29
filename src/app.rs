use piston::input::*;

use opengl_graphics::{GlGraphics};
use opengl_graphics::glyph_cache::{GlyphCache};

use graphics::{clear, ellipse, rectangle};
use graphics::line::Line;
use graphics::text::Text;
use graphics::Transformed;
use graphics::types;

use image::RgbImage;

use std::path::{Path};
use std::f64::consts::{PI};

use super::visual::{Starplot, Dim, Legend};
use super::consts::*;
use super::colors::*;

/// Action that is possible on the application
pub enum Action {
    SaveAsPhoto,
    InvertColor,
    Rotation,
    Quit
}

/// App defines the global application variables for the visualization an Starplot
pub struct App {
    _img: RgbImage, // RGB image
    gl: GlGraphics, // OpenGL drawing backend
    star: Starplot,  // Starplot
    background: types::Color, // Application background
    title: String, // Title of the Visualization
    rotation: f64,
    legend: Legend
}

////////////////////////////////////////////////////////////////////////////////
// Inherent methods
////////////////////////////////////////////////////////////////////////////////

impl App {

    /// Creates a new App instance
    pub fn new(img: RgbImage, gl: GlGraphics) -> App {
        App {_img: img,
             gl: gl, 
             star: Starplot::new(), 
             background: WHITE, 
             title: String::default(), 
             rotation: 0f64,
             legend: Legend::new() }
    }

    /// Defines the Starplot configuration for the visualization
    pub fn def_star(&mut self, star: Starplot) {
        self.star = star;
    }

    // Defines title of the visualization
    pub fn title(&mut self, title: String) {
        self.title = title;
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
    fn get_end_point(size: &[f64; 2], margin: &f64, angle: &f64, val: &f64) -> [f64; 2] {
        [ (size[0] - margin)*val*angle.cos(), 
          -(size[1] - margin)*val*angle.sin() ]
    }

    /// Get label position depending on the value and angle of the associated dimension
    fn get_label_point(size: &[f64; 2], 
                       margin: &f64, 
                       margin_label: &f64, 
                       angle: &f64, 
                       //label_size: usize, 
                       val: &f64) -> [f64; 2] {

        let extra: f64 = 5.0;

        [ (size[0] - margin + margin_label)*val*angle.cos() + extra*angle.cos(), 
          -(size[1] - margin + margin_label)*val*angle.sin() ]
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
        // and the associated label position
        for (i, dim) in self.star.dimensions.iter_mut().enumerate() {            
            let angle: f64 = App::get_angle(&degree_div, i as f64);  

            // initial point is (0,0) taking in count ellipse size
            dim.i_point = [ INITIAL[0], INITIAL[1] ];
            dim.f_point = App::get_end_point( &[ self.star.size_ext, self.star.size_ext ],
                                              &MARGIN, 
                                              &angle, 
                                              &dim.val);

            // get label point for the reference of the legend
            dim.label.pos = App::get_label_point( &[ self.star.size_ext, self.star.size_ext ], 
                                                  &MARGIN, 
                                                  &MARGIN_LABEL, 
                                                  &angle, 
                                                  &dim.val);

            // clone label description to the legend
            self.legend.add_description(dim.label.description.clone());
        }

        // get the contour connections between dimensions
        self.star.contours = App::get_contour(&self.star.dimensions);
    }

    /// Render the Starplot
    pub fn render(&mut self, args: &RenderArgs, font_path: &Path) {  
        // define the position and size of the core ellipse using a square
        let core_square = rectangle::square(0.0, 0.0, self.star.size_sphere);

        // define the position and size of the exterior ellipse using a square
        let ext_square = rectangle::square(0.0, 0.0, self.star.size_ext*1.55);

        // clone Starplot for avoiding borrow error
        let star: Starplot = self.star.clone();

        // get the CharacterCache that describes the used font properties
        let mut glyph = GlyphCache::new(font_path).unwrap();
        
        // clone background Color of App
        let background: types::Color = self.background.clone();

        // clone title of App
        let title: String = self.title.clone();

        // clone rotation of App
        let rot: f64 = self.rotation.clone();

        // clone legend of App
        let legend: Legend = self.legend.clone();

        self.gl.draw(args.viewport(), |c, gl| {
            let initial_transform = c.transform.trans(star.x, star.y) // make the origin at the center of the window
                                               .rot_rad(rot)          // realize a rotation (if configured)
                                               .trans(-0.5*STARPLOT_SPHERE_SIZE, -0.5*STARPLOT_SPHERE_SIZE); // take count the size of the object

            // clear the window
            clear(background, gl);                 
            
            // specify position of title and draw it
            let transform = c.transform.trans(LEGEND_POS_X, MARGIN);
            Text::new_color(star.color, 20).draw(&*title, 
                                                 &mut glyph, 
                                                 &c.draw_state, 
                                                 transform, 
                                                 gl);

            // specify the initial legend position and drawing title
            let legend_transform = c.transform.trans(LEGEND_POS_X, LEGEND_POS_Y);
            Text::new_color(star.color, 12).draw("Legend:", 
                                                 &mut glyph, 
                                                 &c.draw_state, 
                                                 legend_transform, 
                                                 gl);
            
            // specify exterior ellipse
            ellipse::Ellipse::new(background).border(ellipse::Border {color: GRAY, radius: 0.5 })
                                             .draw(ext_square,
                                                   &c.draw_state,
                                                   initial_transform.trans(-star.size_ext*0.775, -star.size_ext*0.775),
                                                   gl);  

            // draw dimensions and labels
            for (i, dim) in star.dimensions.iter().enumerate() {
                Line::new(dim.color, 1.0).draw([dim.i_point[0], dim.i_point[1], dim.f_point[0], dim.f_point[1]], 
                                               &c.draw_state, 
                                               initial_transform, 
                                               gl);

                // specify position of each label and draw it
                let transform = initial_transform.trans(dim.label.pos[0], dim.label.pos[1]); 
                Text::new_color(dim.color, 12).draw(&*i.to_string(), 
                                                    &mut glyph, 
                                                    &c.draw_state, 
                                                    transform, 
                                                    gl);

                                
            }
            // draw contours
            for contour in star.contours.iter() {
                Line::new(star.color, 1.0).draw(*contour, 
                                                &c.draw_state, 
                                                initial_transform, 
                                                gl);
            }

            // Draw the legend list
            for i in 0..legend.description.len() {
                let legend_transform = legend_transform.trans(legend.pos[i][0], legend.pos[i][1]);
                Text::new_color(star.dimensions[i].color, 12).draw(&*legend.description[i], 
                                                                   &mut glyph, 
                                                                   &c.draw_state, 
                                                                   legend_transform, 
                                                                   gl);
            }

            // draw ellipse
            ellipse(star.color, core_square, initial_transform, gl);    
        });
    }

    /// Save visualization as a photo
    pub fn photo(&mut self) {
        unimplemented!()
    }

    /// Inverts the background and Starplot color
    pub fn invert(&mut self) {
        if self.background == WHITE && self.star.color == BLACK {
            self.background = BLACK;
            self.star.color = WHITE;
        } else if self.background == BLACK && self.star.color == WHITE {
            self.background = WHITE;
            self.star.color = BLACK;
        }
    }

    /// Rotates the Starplot 
    pub fn rotation(&mut self) {
        self.rotation += ROTATION_STEP;
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
                    Button::Keyboard(Key::N) => {
                        return Some(Action::InvertColor);
                    }                    
                    _ => {}
                }
            }
            Input::Press(but) => {
                match but {
                    Button::Keyboard(Key::R) => {
                        return Some(Action::Rotation);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        None
    }
}