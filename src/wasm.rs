use std::cell::RefCell;
use std::rc::Rc;
use the_library::graph::{graph_body, GraphSettings};
use crate::ui_manifest::{
    INPUT_A, INPUT_B, INPUT_C, INPUT_EP, 
    INPUT_HEIGHT, INPUT_WIDTH, 
    INPUT_GRAPH_CHAR, INPUT_ABOVE_CHAR, INPUT_BELOW_CHAR
};
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

pub struct Wasm;

impl Wasm {
    // Entry point for JS
    pub fn run() -> JsValue {
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

        {
            // init form edit listener
            // input elements mutate buffer
            let form = form.clone();
            let buffer = buffer.clone();
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
            closure_handle_input.forget();
        }

        let to_js = buffer.clone().borrow().clone();
        serde_wasm_bindgen::to_value(&to_js).unwrap()
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

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Buffer {
    settings: GraphSettings,
    function: [f32; 3],
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            settings: GraphSettings::new(),
            function: [1.0, 2.0, 3.0],
        }
    }

    pub fn update(&mut self, input_id: String, val: String) {
        match input_id.as_str() {
            INPUT_A => {
                if let Ok(val)= val.parse::<f32>() {
                    self.function[0] = val
                }
            },
            INPUT_B => {
                if let Ok(val) = val.parse::<f32>() {
                    self.function[1] = val
                }
            },
            INPUT_C => {
                if let Ok(val) = val.parse::<f32>() {
                    self.function[2] = val
                }
            },
            INPUT_EP => {
                if let Ok(val) = val.parse::<f32>() {
                    self.settings.ep = val
                }
            }
            INPUT_HEIGHT => {
                if let Ok(val) = val.parse::<u8>() {
                    self.settings.height = val
                }
            },
            INPUT_WIDTH => {
                if let Ok(val) = val.parse::<u8>() {
                    self.settings.width = val
                }
            },
            INPUT_GRAPH_CHAR => {
                if let Ok(val) = val.parse::<u8>() {
                    self.settings.graph_char_id = val
                }
            },
            INPUT_ABOVE_CHAR => {
                if let Ok(val) = val.parse::<u8>() {
                    self.settings.above_char_id = val
                }
            },
            INPUT_BELOW_CHAR => {
                if let Ok(val) = val.parse::<u8>() {
                    self.settings.below_char_id = val
                }
            },
            _ => {}
        }
    }
}
