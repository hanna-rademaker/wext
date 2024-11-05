use std::{cell::RefCell, rc::Rc};

use gloo::{events::EventListener, utils::body};
use log::info;
use wext::{
    html::short::{form, h1, input},
    prelude::*,
};

#[derive(Debug)]
struct Item {
    todo: String,
    done: bool,
}

impl Item {
    fn new(todo: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { todo, done: false }))
    }
}

struct State {
    todo_list: Vec<Rc<RefCell<Item>>>,
    _els: Vec<EventListener>,
}

fn main() {
    console_error_panic_hook::set_once();
    log::set_logger(&wasm_bindgen_console_logger::DEFAULT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    body().child(&h1().txt("todos").css("color", "red"));
    let form = form();
    let input_field = input();
    form.child(&input_field);
    body().child(&form);

    let state = Rc::<RefCell<State>>::new_cyclic(|weak| {
        let mut els = Vec::new();
        els.push(weak.event_listener(&form, "submit", move |state, e| {
            e.prevent_default();
            let item = Item::new(input_field.value());
            state.todo_list.push(item);
            input_field.set_value("");
            info!("{:?}", state.todo_list);
        }));
        RefCell::new(State {
            todo_list: Vec::new(),
            _els: els,
        })
    });

    std::mem::forget(state)
}
