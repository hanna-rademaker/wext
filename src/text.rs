pub trait TextExt {
    fn create(s: impl AsRef<str>) -> Self;
    fn text(&self, s: impl AsRef<str>);
}

impl crate::node::NodeExt for web_sys::Text {}

impl TextExt for web_sys::Text {
    fn create(s: impl AsRef<str>) -> Self {
        gloo::utils::document().create_text_node(s.as_ref())
    }
    fn text(&self, s: impl AsRef<str>) {
        self.set_text_content(Some(s.as_ref()))
    }
}

#[macro_export]
macro_rules! text {
    ($($arg:tt)*) => {{
        let s = format!($($arg)*);
        web_sys::Text::create(s)
    }}
}
pub use text;
