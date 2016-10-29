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
            [700, 420]
        )
        .opengl(opengl)
        .build()
        .unwrap();

    // Get the font for the text representation
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("Inconsolata-Regular.ttf");

    // Generate an Starplot
    let mut starplot = Starplot::init(consts::STARPLOT_SPHERE_SIZE,
                                      consts::STARPLOT_SIZE, 
                                      490.0, 
                                      240.0);

    // Add dimensions to the Starplot
    starplot.add_dim(1.0, [0.0, 1.0], "first", colors::RED);
    starplot.add_dim(1.0, [0.0, 1.0], "second", colors::MAGENTA);
    starplot.add_dim(200.0, [0.0, 200.0], "third", colors::ORANGE);
    starplot.add_dim(1.0, [0.0, 1.0], "fourth", colors::YELLOW);
    starplot.add_dim(1.0, [0.0, 1.0], "fifth", colors::GREEN);
    starplot.add_dim(1.0, [0.0, 1.0], "sixth", colors::SEA_GREEN);
    starplot.add_dim(1.0, [0.0, 1.0], "seventh", colors::CYAN);
    starplot.add_dim(1.0, [0.0, 1.0], "eighth", colors::BLUE);
    starplot.add_dim(1.0, [0.0, 1.0], "ninth", colors::DARK_BLUE);
    starplot.add_dim(1.0, [0.0, 1.0], "tenth", colors::VIOLET);

    // Add Starplot to the Application
    let mut app = App::new( RgbImage::new(consts::WINDOW_WIDTH as u32, consts::WINDOW_HEIGHT as u32), 
                            GlGraphics::new(opengl) );
    app.def_star(starplot);

    // Specify title
    app.title("Here comes the title!".to_string());

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
                    Some(Action::SaveAsPhoto) => { app.photo(); }
                    Some(Action::InvertColor) => { app.invert(); }
                    Some(Action::Rotation) => { app.rotation(); }
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