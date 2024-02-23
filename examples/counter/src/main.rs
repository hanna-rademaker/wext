use gloo::utils::{body};
use web_sys::HtmlDivElement;
use web_sys::Text;
use wext::prelude::*;
use wext::html::*;
use wext::text::text;

struct State {
    counter: u64,
    // button: HtmlButtonElement,
}

fn main() {
    console_error_panic_hook::set_once();
    log::set_logger(&wasm_bindgen_console_logger::DEFAULT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);
    body().child(div().child(h1().child(text("Counter"))));
    body().child(&HtmlDivElement::create().child(Text::create(HtmlDivElement::DEFAULT_TAG)));
}
