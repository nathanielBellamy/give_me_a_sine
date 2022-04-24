use the_library::graph;
use the_library::math;
use the_library::text_output;
use the_library::user_input;

fn main() {
    let mut sine_function = math::SineFunction {
        a: 0.0,
        b: 0.0,
        c: 0.0,
    };
    let graph_settings = graph::GraphSettings {
        x_min: -std::f64::consts::PI,
        x_max: std::f64::consts::PI,
        y_min: -1.0,
        y_max: 1.0,
        width: 50,
        height: 20,
        ep: 0.15,
        fill_above: false,
        fill_below: false,
        graph_char: '*',
        shade_graph: graph::ShadeGraph::Integral,
        above_char: 'a',
        below_char: 'b',
    };
    let _integral_approx = graph::IntegralApproximation {
        pos_count: 0,
        neg_count: 0,
        unit_area: 1,
    };
    text_output::intro();
    user_input::get_user_input(&mut sine_function);
    text_output::symbolic_result(&sine_function);
    text_output::graph_rectangle();
    graph::draw_output(&sine_function, &graph_settings);
    text_output::outro();
}
