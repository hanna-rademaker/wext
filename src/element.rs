use web_sys::wasm_bindgen::JsCast;
use web_sys::Node;

pub trait ElementExt {
    fn create() -> Self where Self: JsCast + Sized + Clone;
    fn child(&self, child: impl AsRef<Node>) -> &Self
    where
        Self: AsRef<Node>,
    {
        self.as_ref().append_child(child.as_ref()).unwrap();
        self
    }
}

#[cfg(test)]
pub mod tests {
    use crate::prelude::*;
    use wasm_bindgen_test::*;
    use web_sys::{HtmlButtonElement, HtmlDivElement};

    #[wasm_bindgen_test]
    fn test_element() {
        let b = HtmlButtonElement::create();
        HtmlDivElement::create().child(b);
    }
}
