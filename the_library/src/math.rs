/*
    math.rs
*/

pub struct SineFunction {
    //a sin(b*x + c)
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

pub fn compute_sine_function_at_x(sine_function: &SineFunction, x: f64) -> f64 {
    return sine_function.a * (sine_function.b * x + sine_function.c).sin();
}
