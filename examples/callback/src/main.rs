use gloo::utils::body;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::js_sys::Math::random;
use web_sys::{HtmlDivElement, Node};
use wext::html::short::*;
use wext::prelude::*;
use wext::text::text;

struct ComponentA {
    div: HtmlDivElement,
}

impl ComponentA {
    fn new(parent: &Node) -> Rc<RefCell<Self>> {
        let div = div().css("border", "solid 3px black").child(h2().child(text!("ComponentA")));
        parent.child(&div);
        Rc::new(RefCell::new(Self { div }))
    }
    fn set_color(&mut self, color: &str) {
        self.div.css("background-color", color);
    }
}

struct ComponentB {
    div: HtmlDivElement,
    component_a: Rc<RefCell<ComponentA>>,
}

impl ComponentB {
    fn new(parent: &Node, component_b: Rc<RefCell<ComponentA>>) -> Rc<RefCell<Self>> {
        Rc::<RefCell<Self>>::new_cyclic(|weak| {
            let button = button().child(text!("set random color"));
            let div = div()
                .css("border", "solid 3px black")
                .child(h2().child(text!("ComponentB")))
                .child(&button);
            parent.child(&div);
            weak.event_listener(&button, "click", move |this, _| {
                let colors = ["red", "green", "blue", "pink", "brown"];
                let color = colors[(random() * colors.len() as f64) as usize];

                this.div.css("background-color", color);
                this.component_a.queue(|component_a| component_a.set_color(color));
            })
            .forget();

            RefCell::new(Self {
                div,
                component_a: component_b,
            })
        })
    }
}

fn main() {
    console_error_panic_hook::set_once();
    log::set_logger(&wasm_bindgen_console_logger::DEFAULT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    let parent = body();

    let component_a = ComponentA::new(&parent);
    let component_b = ComponentB::new(&parent, component_a.clone());

    std::mem::forget((component_a, component_b))
}
