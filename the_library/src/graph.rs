/*
    graph.rs
*/
use super::format;
use super::math;

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

pub struct IntegralApproximation {
    //TODO: store method to compute Integral here
    //https://stackoverflow.com/questions/27831944/how-do-i-store-a-closure-in-a-struct-in-rust
    pub pos_count: u64,
    pub neg_count: u64,
    pub unit_area: u64,
}

pub enum ShadeGraph {
    AboveBelow(ShadeAboveBelow),
    Integral,
    NoShade,
}

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
    let mut output: String = format!("{}", '|');
    let mut i: u8 = 0;
    while i < graph_width {
        output = format!("{}{}", output, '~');
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
    let default: char = ' ';
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
