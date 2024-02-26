pub trait HtmlInputExt: AsRef<web_sys::HtmlInputElement> + Clone {
    fn typ(&self, t: impl AsRef<str>) -> Self {
        self.as_ref().set_type(t.as_ref());
        self.clone()
    }
}

impl<T: AsRef<web_sys::HtmlInputElement> + Clone> HtmlInputExt for T {}
