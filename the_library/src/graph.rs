/*
    graph.rs
*/
use super::format;
use super::math;

pub fn draw_output(sine_function: &math::SineFunction) {
    format::empty_line(1);
    format::divider(2);
    println!("{}", math::compute_sine_function_at_x(sine_function, 1.0));
    format::divider(2);
    format::empty_line(1);
}
