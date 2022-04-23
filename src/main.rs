use the_library::graph;
use the_library::math;
use the_library::text_output;
use the_library::user_input;

fn main() {
    text_output::intro();
    let mut sine_function = math::SineFunction {
        a: 0.0,
        b: 0.0,
        c: 0.0,
    };
    user_input::get_user_input(&mut sine_function);
    text_output::result(&sine_function);
    graph::draw_output(&sine_function);
    text_output::outro();
}
