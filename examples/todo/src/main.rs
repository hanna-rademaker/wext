mod item;
mod state;

use crate::state::State;

fn main() {
    console_error_panic_hook::set_once();
    log::set_logger(&wasm_bindgen_console_logger::DEFAULT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    let state = State::new();

    std::mem::forget(state)
}
