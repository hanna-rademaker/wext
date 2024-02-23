use gloo::utils::body;
use web_sys::HtmlDivElement;
use wext::html::short::*;
use wext::prelude::*;
use wext::text::text;

struct State {
    counter: u64,
    // button: HtmlButtonElement,
}

fn main() {
    console_error_panic_hook::set_once();
    log::set_logger(&wasm_bindgen_console_logger::DEFAULT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    body()
        .child(h1().child(text!("Counter")))
        .child(p().child(text!("Button pressed {} times", 0)))
        .child(button().child(text!("+1")));
}
