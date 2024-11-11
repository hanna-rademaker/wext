use std::{cell::RefCell, rc::Rc};

use gloo::events::EventListener;
use web_sys::{HtmlDivElement, HtmlInputElement};
use wext::{html::short::{button, div, input, p}, prelude::*};

pub struct Item {
    pub root: HtmlDivElement,
    pub checkbox: HtmlInputElement,
    delete_callback: Box<dyn Fn()>,
    update_callback: Box<dyn Fn()>,
    _els: Vec<EventListener>,
}

impl Item {
    pub fn new(todo: String, delete_callback: Box<dyn Fn()>, update_callback: Box<dyn Fn()>) -> Rc<RefCell<Self>> {
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
