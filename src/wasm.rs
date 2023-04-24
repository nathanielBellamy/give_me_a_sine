use the_library::graph::{GraphSettings, ShadeGraph, ShadeAboveBelow, graph_body};
use the_library::math::SineFunction;
use the_library::color_square::{ColorSquare, COLOR_SQUARE};
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

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

        let buffer = Buffer::new();
        let buffer = Rc::new(RefCell::new(buffer));
        
        // initial render
        output_element.set_inner_html(
            &graph_body(&buffer.borrow().function, &buffer.borrow().settings)
        );

        {
            // init form edit listener
            // input elements mutate buffer
            let form = form.clone();
            let closure_handle_input = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
                let input = event
                        .target()
                        .unwrap()
                        .dyn_into::<web_sys::HtmlInputElement>()
                        .unwrap();
                let id = input.id(); 

                let val = input.value();
                let buffer = &mut *buffer.borrow_mut();

                buffer.update(id, val);

                output_element.set_inner_html(
                    &graph_body(&buffer.function, &buffer.settings)
                );
            });

            form.add_event_listener_with_callback("input", closure_handle_input.as_ref().unchecked_ref()).unwrap();
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

    pub fn update(&mut self, input_id: String, val: String) {
        match input_id.as_str() {
            "gmas_form_input_a" => {
                self.function.a = val.parse::<f64>().unwrap()
            },
           "gmas_form_input_b" => {
                self.function.b = val.parse::<f64>().unwrap()
            },
           "gmas_form_input_c" => {
                self.function.c = val.parse::<f64>().unwrap()
            },
            "gmas_form_input_ep" => {
                self.settings.ep = val.parse::<f64>().unwrap()
            },
           "gmas_form_input_x_min" => {
                self.settings.x_min = val.parse::<f64>().unwrap()
            },
            "gmas_form_input_height" => {
                self.settings.height = val.parse::<u8>().unwrap()
            },
            "gmas_form_input_width" => {
                self.settings.width = val.parse::<u8>().unwrap()
            },
            "gmas_form_input_graph_char" => {
                let index = val.parse::<u8>().unwrap(); // slider value
                self.settings.graph_char = ColorSquare::by_index(index)
            },
            "gmas_form_input_above_char" => {
                let index = val.parse::<u8>().unwrap(); // slider value
                self.settings.above_char = ColorSquare::by_index(index)
            },
            "gmas_form_input_below_char" => {
                let index = val.parse::<u8>().unwrap(); // slider value
                self.settings.below_char = ColorSquare::by_index(index)
            },
            _ => {}
        }
    }
}
