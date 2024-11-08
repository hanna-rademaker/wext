use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use gloo::{events::EventListener, utils::body};
use uuid::Uuid;
use web_sys::{HtmlDivElement, HtmlInputElement};
use wext::{
    html::short::{button, div, form, h1, input, p},
    node::NodeExt,
    prelude::*,
};

struct Item {
    root: HtmlDivElement,
    _checkbox: HtmlInputElement,
    delete_callback: Box<dyn Fn()>,
    _els: Vec<EventListener>,
}

impl Item {
    fn new(todo: String, delete_callback: Box<dyn Fn()>) -> Rc<RefCell<Self>> {
        Rc::<RefCell<Item>>::new_cyclic(|weak| {
            let root = div();
            let checkbox = input().attr("type", "checkbox");
            let delete_button = button().txt("delete");
            root.child(&div().child(&checkbox).child(&p().txt(todo.as_str())).child(&delete_button));
            let mut els = Vec::new();
            els.push(weak.event_listener(&checkbox, "change", |this, _| {
                this.update();
            }));
            els.push(weak.event_listener(&delete_button, "click", |this, _| {
                (this.delete_callback)();
            }));
            RefCell::new(Self {
                root,
                _checkbox: checkbox,
                delete_callback,
                _els: els,
            })
        })
    }
    fn update(&mut self) {}
}

struct State {
    weak: Weak<RefCell<Self>>,
    todo_list: Vec<(Uuid, Rc<RefCell<Item>>)>,
    container: HtmlDivElement,
    _els: Vec<EventListener>,
}

impl State {
    fn new() -> Rc<RefCell<Self>> {
        Rc::<RefCell<State>>::new_cyclic(|weak| {
            let form = form();
            let input_field = input();
            form.child(&input_field);
            let container = div();
            body().child(&form).child(&container);
            let mut els = Vec::new();
            els.push(weak.event_listener(&form, "submit", move |state, e| {
                e.prevent_default();
                let uuid = Uuid::new_v4();
                let item = Item::new(
                    input_field.value(),
                    state.weak.callback(move |state| {
                        state.todo_list.retain(|(id, _)| id != &uuid);
                        state.update();
                    }),
                );
                state.todo_list.push((uuid, item));
                input_field.set_value("");
                state.update();
            }));
            RefCell::new(State {
                weak: weak.clone(),
                todo_list: Vec::new(),
                container,
                _els: els,
            })
        })
    }
    fn update(&self) {
        self.container.clear();
        for (_, item) in &self.todo_list {
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
