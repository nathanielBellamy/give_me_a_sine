use the_library::graph::{GraphSettings, ShadeGraph, ShadeAboveBelow, graph_body};
use the_library::math::SineFunction;
use the_library::color_square::COLOR_SQUARE;
use wasm_bindgen::prelude::*;
use std::rc::Rc;

#[wasm_bindgen]
pub struct Wasm;

#[wasm_bindgen]
impl Wasm {
    // Entry point for JS
    pub fn run() {
        let output_element = Wasm::output_element();

        let form = Wasm::form();
        let form = Rc::new(form);

        let buffer_read = Buffer::new(); // graph will be drawn based on these values
        let buffer_read = Rc::new(buffer_read);

        let buffer_write = Buffer::new(); // form inputs edit these values
        let buffer_write = Rc::new(buffer_write);

        // form submit copies write buffer onto read buffer and triggers render

        {   
            // init form submit listener
            // copy write buffer to read buffer
            let mut buffer_read = buffer_read.clone();
            let buffer_write = buffer_write.clone();// *Rc::get_mut(&mut buffer_read).unwrap();

            let form = form.clone();
            let closure_submit = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
                if event.key_code() == 13 {
                    let mut buffer_read = *Rc::get_mut(&mut buffer_read).unwrap();
                    buffer_read = *buffer_write.clone();
                    
                    output_element.set_inner_html(
                        &graph_body(&buffer_read.function, &buffer_read.settings)
                    );
                }
            });

            form.add_event_listener_with_callback("keydown", closure_submit.as_ref().unchecked_ref());
            closure_submit.forget();
        }
        {
            // init form edit listener
            // input elements mutate buffer_write
            let mut buffer_write = buffer_write.clone();
            let form = form.clone();
            let closure_handle_input = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
                let mut buffer_write = *Rc::get_mut(&mut buffer_write).unwrap();
                // TODO: pull vaules from form
                buffer_write.settings = GraphSettings {
                    x_min: -std::f64::consts::PI,
                    x_max: std::f64::consts::PI,
                    y_min: -1.0,
                    y_max: 1.0,
                    width: 50,
                    height: 20,
                    ep: 0.15,
                    fill_above: true,
                    fill_below: true,
                    graph_char: *COLOR_SQUARE.red,
                    shade_graph: ShadeGraph::AboveBelow(ShadeAboveBelow::AboveAndBelow),
                    above_char: *COLOR_SQUARE.green,
                    below_char: *COLOR_SQUARE.blue,
                };
                buffer_write.function = SineFunction {
                    a: 1.0,
                    b: 2.0,
                    c: 3.0,
                };
            });

            form.add_event_listener_with_callback("onchange", closure_handle_input.as_ref().unchecked_ref());
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
