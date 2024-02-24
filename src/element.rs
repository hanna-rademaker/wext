pub trait ElementExt {}

impl<T: AsRef<web_sys::Element>> ElementExt for T {}
