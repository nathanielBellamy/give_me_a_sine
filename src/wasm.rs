use the_library::graph;
use the_library::math::SineFunction;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Wasm;

#[wasm_bindgen]
impl Wasm {
    // Entry point for JS
    pub fn run() {
        let settings = graph::GraphSettings {
            x_min: -std::f64::consts::PI,
            x_max: std::f64::consts::PI,
            y_min: -1.0,
            y_max: 1.0,
            width: 50,
            height: 20,
            ep: 0.15,
            fill_above: false,
            fill_below: false,
            graph_char: '*',
            shade_graph: graph::ShadeGraph::NoShade,
            above_char: 'a',
            below_char: 'b',
        };

        let output_element = Wasm::output_element();

        let func: SineFunction = SineFunction::new(1.0, 2.0, 3.0);
        

        output_element.set_inner_html(
            &graph::graph_body(&func, &settings)
        ); 
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
}


