use gloo::utils::document;

pub trait TextExt {
    fn create(s: impl AsRef<str>) -> Self;
}

impl TextExt for web_sys::Text {
    fn create(s: impl AsRef<str>) -> Self {
        document().create_text_node(s.as_ref())
    }
}

pub fn text(s: impl AsRef<str>) -> web_sys::Text {
    web_sys::Text::create(s)
}