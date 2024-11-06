pub trait TextExt {
    fn create(s: impl AsRef<str>) -> Self;
    fn text(&self, s: impl AsRef<str>);
}

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
        use web_sys::Node;
        let s = format!($($arg)*);
        let n: Node = web_sys::Text::create(s).into();
        n
    }}
}
pub use text;
