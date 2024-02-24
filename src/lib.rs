pub mod element;
pub mod html;
pub mod node;
pub mod text;

pub mod prelude {
    pub use crate::element::ElementExt as _;
    pub use crate::html::create_ext::*;
    pub use crate::html::input::HtmlInputExt as _;
    pub use crate::html::HtmlElementExt as _;
    pub use crate::node::NodeExt as _;
    pub use crate::text::TextExt as _;
}

#[cfg(test)]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
