/*
    text_output.rs
*/
use super::format;
use super::math;

pub fn intro() {
    format::empty_line(2);
    format::divider(2);
    format::empty_line(1);
    println!("=====Welcome To=====");
    println!("===Give Me A Sine===");
    format::empty_line(1);
    println!(" y = a * sin(b * x + c)  ");
    format::divider(2);
    format::empty_line(1);
}

pub fn symbolic_result(sine_function: &math::SineFunction) {
    format::empty_line(1);
    format::divider(1);
    println!("  Your sine is:");
    format::empty_line(1);
    println!(
        "      y = {} sin({} * x + {})",
        sine_function.a, sine_function.b, sine_function.c
    );
    format::empty_line(1);
}

pub fn graph_rectangle() {
    println!("  You can see it below graphed on the rectangle [-pi, pi]x[-1, 1].");
    format::divider(1);
    format::empty_line(1);
}

pub fn please_enter_number() {
    format::divider(1);
    println!("Please make sure to enter a number.");
    format::divider(1);
    format::empty_line(1);
}

pub fn outro() {
    format::empty_line(1);
    format::divider(1);
    println!("========End=========");
    format::divider(1);
}
