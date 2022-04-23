/*
    user_input.rs
*/
use super::format;
use super::math;
use super::text_output;

pub fn get_user_input(sine_function: &mut math::SineFunction) {
    handle_user_input(
        "Please enter the outer coefficient 'a': ",
        &mut sine_function.a,
    );
    handle_user_input(
        "Please enter the inner coefficient 'b': ",
        &mut sine_function.b,
    );
    handle_user_input(
        "Please enter the inner constant 'c': ",
        &mut sine_function.c,
    );
}

pub fn handle_user_input(message: &'static str, input_number: &mut f64) {
    //let _unused = input_number;
    format::empty_line(1);
    println!("{}", message);
    let mut input_string = String::new();
    std::io::stdin()
        .read_line(&mut input_string)
        .expect("Please enter a number.");
    let trimmed_input = input_string.trim();
    let input_number_raw = trimmed_input.parse::<f64>();
    match input_number_raw {
        Ok(num) => *input_number = num,
        Err(error) => {
            format::empty_line(1);
            format::divider(1);
            println!("An error occurred: {}", error);
            text_output::please_enter_number();
            handle_user_input(message, input_number);
        }
    };
}
