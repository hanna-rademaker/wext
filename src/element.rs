use web_sys::Node;

pub trait ElementExt {
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
        let b = HtmlButtonElement::create_button();
        HtmlDivElement::create_div().child(b);
    }
}
