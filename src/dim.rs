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

#[derive(Clone)]
pub struct Label {
    pub description: &'static str,
    pub pos: [f64; 2]
}