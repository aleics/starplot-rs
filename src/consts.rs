/// List of the constants used

// Width of the window
pub const WINDOW_WIDTH: f64 = 800.0;

// Height of the window
pub const WINDOW_HEIGHT: f64 = 500.0;

// Center of the Starplot
pub const CENTER: [f64; 2] = [WINDOW_WIDTH/2.0, WINDOW_HEIGHT/2.0];

// Initial value is where the Starplot is positionated (0, 0) taking in count its size
pub const INITIAL: [f64; 2] = [0f64 + (STARPLOT_SPHERE_SIZE/2.0), 0f64 + (STARPLOT_SPHERE_SIZE/2.0)];

// Margin of the window
pub const MARGIN: f64 = 50.0;

// Margin for the label
pub const MARGIN_LABEL: f64 = 10.0;

// Standard size of the sphere of the Starplot
pub const STARPLOT_SPHERE_SIZE: f64 = 20.0;

// Standard sife of the Starplot
pub const STARPLOT_SIZE: f64 = 200.0 + STARPLOT_SPHERE_SIZE;

// Standard Starplot x-position
pub const STARPLOT_POS_X: f64 = (WINDOW_WIDTH/2.0) - (STARPLOT_SPHERE_SIZE/2.0);

// Standard Starplot y-position
pub const STARPLOT_POS_Y: f64 = (WINDOW_HEIGHT/2.0) - (STARPLOT_SPHERE_SIZE/2.0);

// Title position
pub const TITLE_POS: f64 = MARGIN;

// Rotation step defined for the rotation
pub const ROTATION_STEP: f64 = 0.01;

// Legend x-position 
pub const LEGEND_POS_X: f64 = 20.0;

// Legend y-position
pub const LEGEND_POS_Y: f64 = 100.0;

// Legend new line space step
pub const LEGEND_NEWLINE_STEP: f64 = 20.0;