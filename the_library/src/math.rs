/*
    math.rs
*/

pub struct SineFunction {
    //a sin(b*x + c)
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl SineFunction {
    pub fn new(a: f64, b: f64, c: f64) -> SineFunction {
        SineFunction { a, b, c }
    }
}

pub fn compute_sine_function_at_x(sine_function: &SineFunction, x: f64) -> f64 {
    return sine_function.a * (sine_function.b * x + sine_function.c).sin();
}

pub fn x_from_col_index(col_index: u8, graph_width: u8) -> f64 {
    //col_index = 0 -> -pi
    //col_index = graph_width -> pi
    let t: f64 = (col_index as f64) / (graph_width as f64);
    return std::f64::consts::PI * (2.0 * t - 1.0);
}

pub fn y_from_row_index(row_index: u8, graph_height: u8) -> f64 {
    //row_index = 0 -> 1
    //row_index = graph_height -> -1
    let t: f64 = (row_index as f64) / (graph_height as f64);
    return 1.0 - 2.0 * t;
}
