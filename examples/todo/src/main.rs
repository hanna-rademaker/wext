use std::{cell::RefCell, rc::Rc};

use gloo::{events::EventListener, utils::body};
use web_sys::HtmlDivElement;
use wext::{
    html::short::{div, form, h1, input, p},
    prelude::*,
};

#[derive(Debug)]
struct Item {
    root: HtmlDivElement,
    todo: String,
    done: bool,
    _els: Vec<EventListener>,
}

impl Item {
    fn new(todo: String) -> Rc<RefCell<Self>> {
        Rc::<RefCell<Item>>::new_cyclic(|weak| {
            let root = div();
            let checkbox = input().attr("type", "checkbox");
            root.child(&div().child(&checkbox).child(&p().txt(todo.as_str())));
            let mut els = Vec::new();
            els.push(weak.event_listener(&checkbox, "change", |this, _| {
                this.update();
            }));
            RefCell::new(Self {
                root,
                todo,
                done: false,
                _els: els,
            })
        })
    }
    fn update(&mut self) {}
}

struct State {
    todo_list: Vec<Rc<RefCell<Item>>>,
    container: HtmlDivElement,
    _els: Vec<EventListener>,
}

impl State {
    fn new() -> Rc<RefCell<Self>> {
        let state = Rc::<RefCell<State>>::new_cyclic(|weak| {
            let form = form();
            let input_field = input();
            form.child(&input_field);
            let container = div();
            body().child(&form).child(&container);
            let mut els = Vec::new();
            els.push(weak.event_listener(&form, "submit", move |state, e| {
                e.prevent_default();
                let item = Item::new(input_field.value());
                state.todo_list.push(item);
                input_field.set_value("");
                state.update();
            }));
            RefCell::new(State {
                todo_list: Vec::new(),
                container,
                _els: els,
            })
        });
        state
    }
    fn update(&self) {
        self.container.clear();
        for item in &self.todo_list {
            let item = item.borrow();
            self.container.child(&item.root);
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    log::set_logger(&wasm_bindgen_console_logger::DEFAULT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    body().child(&h1().txt("todos").css("color", "red"));
    let state = State::new();

    std::mem::forget(state)
}
