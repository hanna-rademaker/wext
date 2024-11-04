use gloo::utils::body;
use wext::html::short::h1;
use wext::prelude::*;
use wext::svg::short::{circle, rect, svg};
use wext::text::text;

fn main() {
    console_error_panic_hook::set_once();
    log::set_logger(&wasm_bindgen_console_logger::DEFAULT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    body().child(&h1().child(&text!("Svg"))).child(
        &svg()
            .attr("width", "150")
            .attr("height", "150")
            .child(&rect().attr("width", "150").attr("height", "150").css("fill", "green"))
            .child(&circle().attr("cx", "75").attr("cy", "75").attr("r", "50").attr("fill", "orange")),
    );
}
