use std::cell::RefCell;
use std::rc::Rc;
use the_library::color_square::ColorSquare;
use the_library::graph::{graph_body, GraphSettings};
use the_library::math::SineFunction;
use crate::ui_manifest::{
    INPUT_IDS,
    INPUT_A, INPUT_B, INPUT_C, INPUT_EP, 
    INPUT_HEIGHT, INPUT_WIDTH, 
    INPUT_GRAPH_CHAR, INPUT_ABOVE_CHAR, INPUT_BELOW_CHAR
};
use wasm_bindgen::prelude::*;

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
        output_element.set_inner_html(&graph_body(
            &buffer.borrow().function,
            &buffer.borrow().settings,
        ));

        &buffer.borrow().set_ui_initial_values();
        

        {
            // init form edit listener
            // input elements mutate buffer
            let form = form.clone();
            let closure_handle_input =
                Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
                    let input = event
                        .target()
                        .unwrap()
                        .dyn_into::<web_sys::HtmlInputElement>()
                        .unwrap();
                    let id = input.id();

                    let val = input.value();
                    let buffer = &mut *buffer.borrow_mut();

                    buffer.update(id, val);

                    output_element.set_inner_html(&graph_body(&buffer.function, &buffer.settings));
                });

            form.add_event_listener_with_callback(
                "input",
                closure_handle_input.as_ref().unchecked_ref(),
            )
            .unwrap();
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
            INPUT_A => self.function.a = val.parse::<f64>().unwrap(),
            INPUT_B => self.function.b = val.parse::<f64>().unwrap(),
            INPUT_C => self.function.c = val.parse::<f64>().unwrap(),
            INPUT_EP => self.settings.ep = val.parse::<f64>().unwrap(),
            INPUT_HEIGHT => self.settings.height = val.parse::<u8>().unwrap(),
            INPUT_WIDTH => self.settings.width = val.parse::<u8>().unwrap(),
            INPUT_GRAPH_CHAR => {
                let index = val.parse::<u8>().unwrap(); // slider value
                self.settings.graph_char = ColorSquare::by_index(index)
            }
            INPUT_ABOVE_CHAR => {
                let index = val.parse::<u8>().unwrap(); // slider value
                self.settings.above_char = ColorSquare::by_index(index)
            }
            INPUT_BELOW_CHAR => {
                let index = val.parse::<u8>().unwrap(); // slider value
                self.settings.below_char = ColorSquare::by_index(index)
            }
            _ => {}
        }
    }

    pub fn set_ui_initial_values(&self) {
        for id in INPUT_IDS.iter() {
            self.set_ui_initial_value(id.to_string());
        }
    }

    pub fn set_ui_initial_value(&self, input_id: String) {
        let element: web_sys::Element = Wasm::document().get_element_by_id(&input_id)
            .expect("to get element {input_id}");
        let val: String = match input_id.as_str() {
            INPUT_A => self.function.a.to_string(),
            INPUT_B => self.function.b.to_string(),
            INPUT_C => self.function.c.to_string(),
            INPUT_EP => self.settings.ep.to_string(),
            INPUT_HEIGHT => self.settings.height.to_string(),
            INPUT_WIDTH => self.settings.width.to_string(),
            INPUT_GRAPH_CHAR => ColorSquare::to_index_char(self.settings.graph_char).to_string(),
            INPUT_ABOVE_CHAR => ColorSquare::to_index_char(self.settings.above_char).to_string(),
            INPUT_BELOW_CHAR => ColorSquare::to_index_char(self.settings.below_char).to_string(),
            _ => "-1".to_string()
        };
        element.set_attribute("value", &val).expect("to set attribute value on {input_id}"); 
    }
}
