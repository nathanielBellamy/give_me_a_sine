use super::color_square::ColorSquare;
use super::format;
use super::math;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct GraphSettings {
    //TODO: store methods to compute domain_width, range_height
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub width: u8,
    pub height: u8,
    pub ep: f32,
    pub fill_above: bool,
    pub fill_below: bool,
    pub graph_char_id: u8,
    pub above_char_id: u8,
    pub below_char_id: u8,
}

impl GraphSettings {
    pub fn new() -> GraphSettings {
        GraphSettings {
            x_min: -std::f32::consts::PI,
            x_max: std::f32::consts::PI,
            y_min: -1.0,
            y_max: 1.0,
            width: 32,
            height: 20,
            ep: 0.25,
            fill_above: false,
            fill_below: false,
            graph_char_id: 3,
            above_char_id: 1,
            below_char_id: 4,
        }
    }
}

pub struct IntegralApproximation {
    //TODO: store method to compute Integral here
    //https://stackoverflow.com/questions/27831944/how-do-i-store-a-closure-in-a-struct-in-rust
    pub pos_count: u64,
    pub neg_count: u64,
    pub unit_area: u64,
}

pub fn draw_output(sine_function: &[f32; 3], graph_settings: &GraphSettings) {
    format::empty_line(1);
    horizontal_boundary(graph_settings.width);
    graph_body(sine_function, graph_settings);
    horizontal_boundary(graph_settings.width);
    format::empty_line(1);
}

fn horizontal_boundary(graph_width: u8) {
    let mut output: String = String::new();
    let mut i: u8 = 0;
    while i < graph_width {
        output = format!("{}{}", output, '~');
        i += 1;
    }
    println!("{}", output);
}

pub fn graph_body(sine_function: &[f32; 3], graph_settings: &GraphSettings) -> String {
    let mut output: String = "".to_string();
    let mut row_index: u8 = 0;
    while row_index < graph_settings.height {
        output.push_str(&graph_row(sine_function, row_index, graph_settings));
        row_index += 1;
    }

    output
}

fn graph_row(
    sine_function: &[f32; 3],
    row_index: u8,
    graph_settings: &GraphSettings,
) -> String {
    let mut output: String = String::new();
    let mut col_index: u8 = 0;
    let y: f32 = math::y_from_row_index(row_index, graph_settings.height);
    while col_index < graph_settings.width {
        output = format!(
            "{}{}",
            output,
            get_char(sine_function, row_index, col_index, y, graph_settings)
        );
        col_index += 1;
    }
    output = format!("{}{}{}", "<div class='gmas_graph_row'>", output, "</div>");
    // println!("{}", output);
    output
}

fn get_char(
    sine_function: &[f32; 3],
    _row_index: u8,
    col_index: u8,
    y: f32,
    graph_settings: &GraphSettings,
) -> char {
    let output: char;
    let x: f32 = math::x_from_col_index(col_index, graph_settings.width);
    let f_x: f32 = math::compute_sine_function_at_x(sine_function, x);
    let on_graph: bool = on_graph(f_x, y, graph_settings.ep);
    if on_graph == true {
        output = ColorSquare::by_id(graph_settings.graph_char_id);
    } else {
        output = get_non_graph_char(x, y, f_x, graph_settings);
    }
    return output;
}

fn get_non_graph_char(_x: f32, y: f32, f_x: f32, graph_settings: &GraphSettings) -> char {
    let color_id = if y > f_x {
        graph_settings.above_char_id
    } else {
        graph_settings.below_char_id
    };
    ColorSquare::by_id(color_id)
}

fn on_graph(f_x: f32, y: f32, ep: f32) -> bool {
    return (f_x - y).abs() < ep;
}
