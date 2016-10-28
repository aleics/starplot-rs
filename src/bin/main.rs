extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate opengl_graphics;
extern crate find_folder;
extern crate image;
extern crate starplot;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::{PistonWindow};
use opengl_graphics::{ GlGraphics, OpenGL };
use image::RgbImage;

use starplot::colors;
use starplot::consts;
use starplot::visual::Starplot;
use starplot::app::{Action, App};

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Piston window
    let mut window: PistonWindow = WindowSettings::new(
            "starplot",
            [consts::WINDOW_SIZE as u32, consts::WINDOW_SIZE as u32]
        )
        .opengl(opengl)
        .build()
        .unwrap();

    // Get the font for the text representation
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("Inconsolata-Regular.ttf");

    // Generate an Starplot
    let mut starplot = Starplot::init(consts::STARPLOT_SIZE, 
                                      consts::STARPLOT_POS_X, 
                                      consts::STARPLOT_POS_Y);

    // Add dimensions to the Starplot
    starplot.add_dim(0.2, [0.0, 1.0], "first (0.2)", colors::RED);
    starplot.add_dim(0.8, [0.0, 1.0], "second (0.8)", colors::MAGENTA);
    starplot.add_dim(50.0, [0.0, 200.0], "third (50.0)", colors::ORANGE);
    starplot.add_dim(0.4, [0.0, 1.0], "fourth (0.4)", colors::YELLOW);
    starplot.add_dim(0.7, [0.0, 1.0], "fifth (0.7)", colors::GREEN);
    starplot.add_dim(0.6, [0.0, 1.0], "sixth (0.6)", colors::SEA_GREEN);
    starplot.add_dim(0.5, [0.0, 1.0], "seventh (0.5)", colors::CYAN);
    starplot.add_dim(0.3, [0.0, 1.0], "eighth (0.3)", colors::BLUE);
    starplot.add_dim(0.5, [0.0, 1.0], "ninth (0.5)", colors::DARK_BLUE);
    starplot.add_dim(0.6, [0.0, 1.0], "tenth (0.6)", colors::VIOLET);

    // Add Starplot to the Application
    let mut app = App::new( RgbImage::new(consts::WINDOW_SIZE as u32, consts::WINDOW_SIZE as u32), 
                            GlGraphics::new(opengl) );
    app.def_star(starplot);

    // Preprocessing
    app.preproc();

    let mut events = window.events();
    loop { // listen to events
        let e = events.next(&mut window);
        match e {

            Some(Event::Render(r)) => {            
                app.render(&r, &font.as_path()); // render the Application
            }

            Some(Event::Input(inp)) => {
                let action = app.on_input(inp); // handle user input
                match action {
                    Some(Action::Quit) => { break; } // exit
                    Some(Action::SaveAsPhoto) => { }
                    _ => {}
                } 
            }

            Some(Event::Update(_)) => {}

            Some(Event::AfterRender(_)) => {}

            Some(Event::Idle(_)) => {}

            None => {}

        }
    }
}