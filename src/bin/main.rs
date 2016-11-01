extern crate starplot;

use starplot::colors;
use starplot::consts;
use starplot::visual::Starplot;
use starplot::app::App;

fn main() {
    // Add Starplot to the Application
    let mut app = App::new(700, 420);

    // Generate an Starplot
    let mut starplot = Starplot::init(consts::STARPLOT_SPHERE_SIZE,
                                      consts::STARPLOT_SIZE, 
                                      490.0, 
                                      240.0);

    // Add dimensions to the Starplot
    starplot.add_dim(0.2, [0.0, 1.0], "first".to_string(), colors::RED);
    starplot.add_dim(0.8, [0.0, 1.0], "second".to_string(), colors::MAGENTA);
    starplot.add_dim(50.0, [0.0, 200.0], "third".to_string(), colors::ORANGE);
    starplot.add_dim(1.0, [0.0, 1.0], "fourth".to_string(), colors::YELLOW);
    starplot.add_dim(1.0, [0.0, 1.0], "fifth".to_string(), colors::GREEN);
    starplot.add_dim(0.6, [0.0, 1.0], "sixth".to_string(), colors::SEA_GREEN);
    starplot.add_dim(0.5, [0.0, 1.0], "seventh".to_string(), colors::CYAN);
    starplot.add_dim(0.8, [0.0, 1.0], "eighth".to_string(), colors::BLUE);
    starplot.add_dim(0.5, [0.0, 1.0], "ninth".to_string(), colors::DARK_BLUE);
    starplot.add_dim(1.0, [0.0, 1.0], "tenth".to_string(), colors::VIOLET);

    app.def_star(starplot);

    // Specify title
    app.title("Here comes the title!".to_string());

    // Preprocessing
    app.preproc();

    // Start
    app.start();
}