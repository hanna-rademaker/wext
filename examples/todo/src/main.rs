use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use gloo::{events::EventListener, utils::body};
use uuid::Uuid;
use web_sys::{HtmlDivElement, HtmlInputElement, HtmlSpanElement};
use wext::{
    html::short::{button, div, form, h1, input, p, span},
    node::NodeExt,
    prelude::*,
};

pub struct Item {
    pub root: HtmlDivElement,
    pub checkbox: HtmlInputElement,
    delete_callback: Box<dyn Fn()>,
    update_callback: Box<dyn Fn()>,
    _els: Vec<EventListener>,
}

impl Item {
    fn new(todo: String, delete_callback: Box<dyn Fn()>, update_callback: Box<dyn Fn()>) -> Rc<RefCell<Self>> {
        Rc::<RefCell<Item>>::new_cyclic(|weak| {
            let root = div();
            let checkbox = input().attr("type", "checkbox");
            let delete_button = button().txt("delete");
            root.child(&div().class("item").child(&checkbox).child(&p().txt(todo.as_str())).child(&delete_button));
            let mut els = Vec::new();
            els.push(weak.event_listener(&checkbox, "change", |this, _| {
                this.update();
                (this.update_callback)();
            }));
            els.push(weak.event_listener(&delete_button, "click", |this, _| {
                (this.delete_callback)();
            }));
            RefCell::new(Self {
                root,
                checkbox,
                delete_callback,
                update_callback,
                _els: els,
            })
        })
    }
    fn update(&mut self) {}
}

enum Filter {
    All,
    Active,
    Completed,
}

pub struct State {
    weak: Weak<RefCell<Self>>,
    todo_list: Vec<(Uuid, Rc<RefCell<Item>>)>,
    container: HtmlDivElement,
    filter: Filter,
    filters_div: HtmlDivElement,
    counter: HtmlSpanElement,
    _els: Vec<EventListener>,
}

impl State {
    fn new() -> Rc<RefCell<Self>> {
        Rc::<RefCell<State>>::new_cyclic(|weak| {
            let title = h1().txt("todos").class("title");
            let form = form();
            let input_field = input().class("input-field").attr("placeholder", "What needs to be done?");
            form.child(&input_field);
            let container = div();
            let button_all = button().txt("All").class("all");
            let button_active = button().txt("Active").class("active");
            let button_completed = button().txt("Completed").class("completed");
            let filters_div = div().attr("class", "filter all");
            filters_div.child(&button_all).child(&button_active).child(&button_completed);
            let counter = span().class("counter");
            let footer = div().class("footer").child(&counter).child(&filters_div);
            let root = div().class("root").child(&title).child(&form).child(&container).child(&footer);
            body().child(&root);
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
                    state.weak.callback(move |state| {
                        state.update();
                    }),
                );
                state.todo_list.push((uuid, item));
                input_field.set_value("");
                state.update();
            }));
            els.push(weak.event_listener(&button_all, "click", |state, _| {
                state.filter = Filter::All;
                state.update();
            }));
            els.push(weak.event_listener(&button_active, "click", |state, _| {
                state.filter = Filter::Active;
                state.update();
            }));
            els.push(weak.event_listener(&button_completed, "click", |state, _| {
                state.filter = Filter::Completed;
                state.update();
            }));
            RefCell::new(State {
                weak: weak.clone(),
                todo_list: Vec::new(),
                container,
                filter: Filter::All,
                filters_div,
                counter,
                _els: els,
            })
        })
    }
    fn update(&self) {
        self.container.clear();
        match self.filter {
            Filter::All => {
                for (_, item) in &self.todo_list {
                    let item = item.borrow();
                    self.container.child(&item.root);
                }
                self.filters_div.attr("class", "filter all");
            }
            Filter::Active => {
                for (_, item) in &self.todo_list {
                    let item = item.borrow();
                    if !item.checkbox.checked() {
                        self.container.child(&item.root);
                    }
                }
                self.filters_div.attr("class", "filter active");
            }
            Filter::Completed => {
                for (_, item) in &self.todo_list {
                    let item = item.borrow();
                    if item.checkbox.checked() {
                        self.container.child(&item.root);
                    }
                }
                self.filters_div.attr("class", "filter completed");
            }
        }
        self.counter.txt(self.counter_of_active_items().as_str());
    }
    fn counter_of_active_items(&self) -> String {
        let mut c = 0usize;
        for (_, item) in &self.todo_list {
            let item = item.borrow();
            if !item.checkbox.checked() {
                c += 1;
            }
        }
        if c == 1 {
            return format!("{} item left", c);
        }
        format!("{} items left", c)
    }
}

fn main() {
    console_error_panic_hook::set_once();
    log::set_logger(&wasm_bindgen_console_logger::DEFAULT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    let state = State::new();

    std::mem::forget(state)
}
