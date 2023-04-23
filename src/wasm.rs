use the_library::graph::{GraphSettings, ShadeGraph, ShadeAboveBelow, graph_body};
use the_library::math::SineFunction;
use the_library::color_square::COLOR_SQUARE;
use wasm_bindgen::prelude::*;
use std::rc::Rc;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Wasm;

#[wasm_bindgen]
impl Wasm {
    // Entry point for JS
    pub fn run() {
        let output_element = Wasm::output_element();

        let form = Wasm::form();
        let form = Rc::new(form);

        let buffer = Buffer::new(); // graph will be drawn based on these values
        let buffer = Rc::new(buffer);

        // initial render
        output_element.set_inner_html(
            &graph_body(&buffer.function, &buffer.settings)
        );

        // form submit copies write buffer onto read buffer and triggers render

        {   
            // init form submit listener
            // render from buffer

            let form = form.clone();
            let buffer = buffer.clone();
            let closure_submit = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
                // if event.key_code() == 13 {
                    output_element.set_inner_html(
                        &graph_body(&buffer.function, &buffer.settings)
                    );
                // }
            });

            form.add_event_listener_with_callback("keydown", closure_submit.as_ref().unchecked_ref());
            closure_submit.forget();
        }
        {
            // init form edit listener
            // input elements mutate buffer
            let mut buffer = buffer.clone();
            let form = form.clone();
            let closure_handle_input = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
                unsafe {
                   console_log!("HEREHER")
                }
                // let mut buffer = *Rc::get_mut(&mut buffer).unwrap();
                // TODO: pull vaules from form
                // buffer.settings = GraphSettings {
                //     x_min: -std::f64::consts::PI,
                //     x_max: std::f64::consts::PI,
                //     y_min: -1.0,
                //     y_max: 1.0,
                //     width: 50,
                //     height: 20,
                //     ep: 0.15,
                //     fill_above: true,
                //     fill_below: true,
                //     graph_char: *COLOR_SQUARE.red,
                //     shade_graph: ShadeGraph::AboveBelow(ShadeAboveBelow::AboveAndBelow),
                //     above_char: *COLOR_SQUARE.purple,
                //     below_char: *COLOR_SQUARE.orange,
                // };
                // buffer.function = SineFunction {
                //     a: 1.0,
                //     b: 2.0,
                //     c: 3.0,
                // };
            });

            form.add_event_listener_with_callback("change", closure_handle_input.as_ref().unchecked_ref()).unwrap();
            closure_handle_input.forget()
        }
    }
}


impl Wasm {
    fn window() -> web_sys::Window {
        web_sys::window().expect("no global `window` exists")
    }

    fn document() -> web_sys::Document {
        Wasm::window()
            .document()
            .expect("should have a document on window")
    }

    fn output_element() -> web_sys::Element { 
        Wasm::document()
            .get_element_by_id("give_me_a_sine_output")
            .expect("unable to find canvas element")
    }

    fn form() -> web_sys::Element {
        Wasm::document()
            .get_element_by_id("give_me_a_sine_form")
            .expect("unable to find form element")
    }
}

#[derive(Clone, Copy)]
pub struct Buffer {
    settings: GraphSettings,
    function: SineFunction,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            settings: GraphSettings::new(),
            function: SineFunction::new(1.0, 2.0, 3.0),
        }
    }

}
