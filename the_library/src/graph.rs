use super::color_square::COLOR_SQUARE;
use super::format;
use super::math;

#[derive(Clone, Copy)]
pub struct GraphSettings {
    //TODO: store methods to compute domain_width, range_height
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub width: u8,
    pub height: u8,
    pub ep: f64,
    pub fill_above: bool,
    pub fill_below: bool,
    pub graph_char: char,
    pub shade_graph: ShadeGraph,
    pub above_char: char,
    pub below_char: char,
}

impl GraphSettings {
    pub fn new() -> GraphSettings {
        GraphSettings {
            x_min: -std::f64::consts::PI,
            x_max: std::f64::consts::PI,
            y_min: -1.0,
            y_max: 1.0,
            width: 32,
            height: 20,
            ep: 0.25,
            fill_above: false,
            fill_below: false,
            graph_char: *COLOR_SQUARE.blue,
            shade_graph: ShadeGraph::AboveBelow(ShadeAboveBelow::AboveAndBelow),
            above_char: *COLOR_SQUARE.white,
            below_char: *COLOR_SQUARE.green,
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

#[derive(Clone, Copy)]
pub enum ShadeGraph {
    AboveBelow(ShadeAboveBelow),
    Integral,
    NoShade,
}

#[derive(Clone, Copy)]
pub enum ShadeAboveBelow {
    Above,
    Below,
    AboveAndBelow,
}

pub fn draw_output(sine_function: &math::SineFunction, graph_settings: &GraphSettings) {
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

pub fn graph_body(sine_function: &math::SineFunction, graph_settings: &GraphSettings) -> String {
    let mut output: String = "".to_string();
    let mut row_index: u8 = 0;
    while row_index < graph_settings.height {
        output.push_str(&graph_row(sine_function, row_index, graph_settings));
        row_index += 1;
    }

    output
}

fn graph_row(
    sine_function: &math::SineFunction,
    row_index: u8,
    graph_settings: &GraphSettings,
) -> String {
    let mut output: String = String::new();
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
    output = format!("{}{}{}", "<div class='gmas_graph_row'>", output, "</div>");
    // println!("{}", output);
    output
}

fn get_char(
    sine_function: &math::SineFunction,
    _row_index: u8,
    col_index: u8,
    y: f64,
    graph_settings: &GraphSettings,
) -> char {
    let output: char;
    let x: f64 = math::x_from_col_index(col_index, graph_settings.width);
    let f_x: f64 = math::compute_sine_function_at_x(sine_function, x);
    let on_graph: bool = on_graph(f_x, y, graph_settings.ep);
    if on_graph == true {
        output = graph_settings.graph_char;
    } else {
        output = get_non_graph_char(x, y, f_x, graph_settings);
    }
    return output;
}

fn get_non_graph_char(_x: f64, y: f64, f_x: f64, graph_settings: &GraphSettings) -> char {
    let default: char = *COLOR_SQUARE.white;
    match &graph_settings.shade_graph {
        ShadeGraph::AboveBelow(shade_above_below) => match shade_above_below {
            ShadeAboveBelow::Above => {
                if y > f_x {
                    graph_settings.above_char
                } else {
                    default
                }
            }
            ShadeAboveBelow::Below => {
                if y < f_x {
                    graph_settings.below_char
                } else {
                    default
                }
            }
            ShadeAboveBelow::AboveAndBelow => {
                if y > f_x {
                    graph_settings.above_char
                } else {
                    graph_settings.below_char
                }
            }
        },
        ShadeGraph::Integral => {
            if y > 0.0 {
                if y < f_x {
                    '+'
                } else {
                    default
                }
            } else {
                if y > f_x {
                    '-'
                } else {
                    default
                }
            }
        }
        ShadeGraph::NoShade => default,
    }
}

fn on_graph(f_x: f64, y: f64, ep: f64) -> bool {
    return (f_x - y).abs() < ep;
}
