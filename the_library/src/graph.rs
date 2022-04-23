/*
    graph.rs
*/
use super::format;
use super::math;

pub struct GraphSettings {
    pub width: u8,
    pub height: u8,
}

pub fn draw_output(sine_function: &math::SineFunction, graph_settings: &GraphSettings) {
    format::empty_line(1);
    horizontal_boundary(graph_settings.width);
    graph_body(sine_function, graph_settings);
    horizontal_boundary(graph_settings.width);
    format::empty_line(1);
}

fn horizontal_boundary(graph_width: u8) {
    let mut output: String = format!("{}", '|');
    let mut i: u8 = 0;
    while i < graph_width {
        output = format!("{}{}", output, "-");
        i += 1;
    }
    output = format!("{}{}", output, '|');
    println!("{}", output);
}

fn graph_body(sine_function: &math::SineFunction, graph_settings: &GraphSettings) {
    let mut row_index: u8 = 0;
    while row_index < graph_settings.height {
        graph_row(sine_function, row_index, graph_settings);
        row_index += 1;
    }
}

fn graph_row(sine_function: &math::SineFunction, row_index: u8, graph_settings: &GraphSettings) {
    let mut output: String = String::from('|');
    let mut col_index: u8 = 0;
    let y: f64 = math::y_from_row_index(row_index, graph_settings.height);
    while col_index < graph_settings.width {
        output = format!(
            "{}{}",
            output,
            get_char(sine_function, row_index, col_index, y, graph_settings)
        );
        col_index += 1;
    }
    output = format!("{}{}", output, '|');
    println!("{}", output);
}

fn get_char(
    sine_function: &math::SineFunction,
    _row_index: u8,
    col_index: u8,
    y: f64,
    graph_settings: &GraphSettings,
) -> char {
    let output: char;
    let ep: f64 = 0.17;
    let x: f64 = math::x_from_col_index(col_index, graph_settings.width);
    let on_graph: bool = on_graph(sine_function, x, y, ep);
    if on_graph == true {
        output = '*';
    } else {
        output = ' ';
    }
    return output;
}

fn on_graph(sine_function: &math::SineFunction, x: f64, y: f64, ep: f64) -> bool {
    let f_x: f64 = math::compute_sine_function_at_x(sine_function, x);
    return (f_x - y).abs() < ep;
}
