use gloo::events::EventListener;
use gloo::utils::body;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Node;
use wext::html::short::*;
use wext::prelude::*;
use wext::text::text;

struct State {
    count: u8,
    info: Node,
    _els: Vec<EventListener>,
}

impl State {
    fn process(&mut self, increment: bool) {
        match increment {
            true => self.count = self.count.wrapping_add(1),
            false => self.count = self.count.wrapping_sub(1),
        }
        self.update()
    }
    fn update(&mut self) {
        self.info.overwrite(text!("{}", self.count));
    }
}

fn main() {
    console_error_panic_hook::set_once();
    log::set_logger(&wasm_bindgen_console_logger::DEFAULT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    body().child(&h1().txt("Counter").css("color", "grey"));

    let state = Rc::<RefCell<State>>::new_cyclic(|weak| {
        let inc = button().txt("+1");
        let dec = button().txt("-1");
        let info = text!("0");
        body().child(&dec).child(&inc).child(&p().child(&text!("Count: ")).child(&info));

        let mut els = Vec::new();
        els.push(weak.event_listener(&inc, "click", |state, _| state.process(true)));
        els.push(weak.event_listener(&dec, "click", |state, _| state.process(false)));

        RefCell::new(State { count: 0, info, _els: els })
    });

    std::mem::forget(state)
}
