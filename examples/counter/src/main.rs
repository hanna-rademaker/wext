use gloo::events::EventListener;
use gloo::utils::body;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Text;
use wext::html::short::*;
use wext::prelude::*;
use wext::text::text;

struct State {
    count: u8,
    info: Text,
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

    body().child(h1().child(text!("Counter")));

    let state = Rc::<RefCell<State>>::new_cyclic(|weak| {
        let inc = button().child(text!("+1")).clone();
        let dec = button().child(text!("-1")).clone();
        let info = text!("0");
        body().child(&dec).child(&inc).child(p().child(text!("Count: ")).child(&info));

        let mut els = Vec::new();
        let weak_clone = weak.clone();
        els.push(EventListener::new(&inc, "click", move |_| {
            if let Some(state) = weak_clone.upgrade() {
                state.borrow_mut().process(true)
            }
        }));
        let weak_clone = weak.clone();
        els.push(EventListener::new(&dec, "click", move |_| {
            if let Some(state) = weak_clone.upgrade() {
                state.borrow_mut().process(false)
            }
        }));

        RefCell::new(State { count: 0, info, _els: els })
    });

    std::mem::forget(state)
}
