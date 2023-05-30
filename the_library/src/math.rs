pub fn compute_sine_function_at_x(sine_function: &[f32; 3], x: f32) -> f32 {
    return sine_function[0] * (sine_function[1] * x + sine_function[2]).sin();
}

pub fn x_from_col_index(col_index: u8, graph_width: u8) -> f32 {
    //col_index = 0 -> -pi
    //col_index = graph_width -> pi
    let t: f32 = (col_index as f32) / (graph_width as f32);
    return std::f32::consts::PI * (2.0 * t - 1.0);
}

pub fn y_from_row_index(row_index: u8, graph_height: u8) -> f32 {
    //row_index = 0 -> 1
    //row_index = graph_height -> -1
    let t: f32 = (row_index as f32) / (graph_height as f32);
    return 1.0 - 2.0 * t;
}
