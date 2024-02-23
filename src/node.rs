pub trait NodeExt: AsRef<web_sys::Node> {
    /// shorthand for `self.append_child(&child).unwrap()`
    fn child(&self, child: impl AsRef<web_sys::Node>) -> &Self {
        self.as_ref().append_child(child.as_ref()).unwrap();
        self
    }
    /// removes all child nodes
    fn clear(&self) {
        let node = self.as_ref();
        while let Some(child) = node.last_child() {
            node.remove_child(&child).unwrap();
        }
    }
    /// shorthand for `self.parent_node().unwrap().replace_child(&new, self).unwrap()`
    fn replace(&self, new: impl AsRef<web_sys::Node>) {
        self.as_ref().parent_node().unwrap().replace_child(new.as_ref(), self.as_ref()).unwrap();
    }
    /// like [Self::replace], but also overwrites `self` with the new element and returns the original `self`.
    fn overwrite(&mut self, new: Self) -> Self
    where
        Self: Sized,
    {
        self.replace(new.as_ref());
        std::mem::replace(self, new)
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
