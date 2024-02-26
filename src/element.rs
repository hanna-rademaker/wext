pub trait ElementExt: AsRef<web_sys::Element> + Clone {
    /// Add `c` to the class list
    fn class(&self, c: impl AsRef<str>) -> Self {
        self.as_ref().class_list().add_1(c.as_ref()).unwrap();
        self.clone()
    }
    /// Set the attribute `name` to the specified value
    fn attr(&self, name: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.as_ref().set_attribute(name.as_ref(), value.as_ref()).unwrap();
        self.clone()
    }
}

impl<T: AsRef<web_sys::Element> + Clone> ElementExt for T {}
