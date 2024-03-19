pub mod callback;
pub mod element;
pub(crate) mod element_macros;
pub mod event_listener;
pub mod html;
pub mod node;
pub mod svg;
pub mod text;

pub mod prelude {
    pub use crate::callback::WeakRcRefCellCallbackExt as _;
    pub use crate::element::ElementExt as _;
    pub use crate::event_listener::EventListenerExt as _;
    pub use crate::event_listener::WeakRcRefCellEventListenerExt as _;
    pub use crate::html::create_ext::*;
    pub use crate::html::input::HtmlInputExt as _;
    pub use crate::html::HtmlElementExt as _;
    pub use crate::node::NodeExt as _;
    pub use crate::svg::SvgElementExt as _;
    pub use crate::text::TextExt as _;
}

#[cfg(test)]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
