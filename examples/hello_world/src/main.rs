use gloo::utils::body;
use wext::html::short::{div, h1};
use wext::node::NodeExt;

fn main() {
    console_error_panic_hook::set_once();
    log::set_logger(&wasm_bindgen_console_logger::DEFAULT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    log::info!("Hello World!");

    let root = div();
    let heading = h1().txt("Hello World");
    root.child(&heading);
    body().child(&root);
}
