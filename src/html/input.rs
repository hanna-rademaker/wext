pub trait HtmlInputExt: AsRef<web_sys::HtmlInputElement> {
    fn typ(&self, t: impl AsRef<str>) -> &Self {
        self.as_ref().set_type(t.as_ref());
        self
    }
}

impl<T: AsRef<web_sys::HtmlInputElement>> HtmlInputExt for T {}
