extern crate starplot;

use starplot::app::App;
use std::env;

fn main() {
    // Collect the arguments
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 { // if no argument
        panic!("Send the absolut path of the JSON configuration file as an argument");
    }
    // Get the introduced filepath
    let filepath: String = args[1].clone();

    // Creates a new Application instance and read the configuration file
    let mut app: App = App::new(700, 420);
    app.read_conf(filepath);

    // Preprocessing
    app.preproc();

    // Start
    app.start();
}