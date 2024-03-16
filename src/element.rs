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
    /// Set the attribute `name` to the specified integer value
    fn iattr(&self, name: impl AsRef<str>, value: impl Into<i32>) -> Self {
        let s = value.into().to_string();
        self.as_ref().set_attribute(name.as_ref(), &s).unwrap();
        self.clone()
    }
    /// Set or unset boolean attribute `name`
    fn battr(&self, name: impl AsRef<str>, value: bool) -> Self {
        match value {
            true => self.as_ref().set_attribute(name.as_ref(), "").unwrap(),
            false => self.as_ref().remove_attribute(name.as_ref()).unwrap(),
        }
        self.clone()
    }
}

impl<T: AsRef<web_sys::Element> + Clone> ElementExt for T {}
