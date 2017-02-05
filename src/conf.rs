use std::fs::File;
use std::path::Path;
use std::io::Read;
use serde_json;

use super::app::GObjects;
use super::visual::Starplot;

/// DimConf defines the configuration structure for the dimensions
/// of an Starplot
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DimConf {
    val: f64,
    range: [f64; 2],
    label: String,
    color: [f32; 4]        
}

/// StarConf defines the configuration structure of an Starplot
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StarConf {
    title: String,
    size_core: f64,
    size_ext: f64,
    pos_x: f64,
    pos_y: f64, 
    dimensions: Vec<DimConf>
}

////////////////////////////////////////////////////////////////////////////////
// Inherent methods
////////////////////////////////////////////////////////////////////////////////

impl StarConf {

    /// Reads the configuration from a JSON file
    pub fn read_from_file(path: &Path) -> Option<GObjects> {
        // open the given file
        let mut file = match File::open(path) { 
            Err(e) => panic!("couldn't open {}: {}", path.to_str().unwrap(), e),
            Ok(file) => file,
        };

        let mut content = String::new();

        // read the file's content
        match file.read_to_string(&mut content) {
            Err(e) => panic!("couldn't read {}: {}", path.to_str().unwrap(), e),
            Ok(_) => {
                // deserialize the content in a StarConf variable
                let starconf: StarConf = match serde_json::from_str(content.as_str()) {
                    Err(e) => panic!("format exception from '{}'. error: {}.\n {}", path.to_str().unwrap(), e, content),
                    Ok(objects) => objects,
                };
                
                // if the given dimensions are 0, cancel visualization
                if starconf.dimensions.len() == 0 {
                    println!("Dimension configuration couldn't be process it, or it's not specified!");
                    return None;
                }

                // create a new instance of GObjects and initialize the Starplot variable
                let mut gobjects: GObjects = GObjects::new();
                gobjects.star = Starplot::init(starconf.size_core, 
                                               starconf.size_ext, 
                                               starconf.pos_x, 
                                               starconf.pos_y);

                gobjects.title = starconf.title;
                // add the dimensions' configuration to the Starplot
                for dim in starconf.dimensions {
                    gobjects.star.add_dim(dim.val, 
                                          dim.range, 
                                          dim.label, 
                                          dim.color);
                }
                return Some(gobjects);
            },
        }
    }
}
