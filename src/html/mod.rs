pub mod input;
pub trait HtmlElementExt: AsRef<web_sys::HtmlElement> + Clone {
    fn create(tag: impl AsRef<str>) -> Self
    where
        Self: web_sys::wasm_bindgen::JsCast,
    {
        use web_sys::wasm_bindgen::JsCast;
        gloo::utils::document()
            .create_element_ns(Some("http://www.w3.org/1999/xhtml"), tag.as_ref())
            .unwrap()
            .dyn_into()
            .unwrap()
    }
    fn css(&self, property: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.as_ref().style().set_property(property.as_ref(), value.as_ref()).unwrap();
        self.clone()
    }
    fn ocss(&self, property: impl AsRef<str>, value: Option<&str>) -> Self {
        let style = self.as_ref().style();
        let property = property.as_ref();
        match value {
            None => {
                style.remove_property(property).unwrap();
            }
            Some(v) => style.set_property(property, v).unwrap(),
        }
        self.clone()
    }
}

impl<T: AsRef<web_sys::HtmlElement> + Clone> HtmlElementExt for T {}

pub(crate) type ThisBase = web_sys::HtmlElement;
pub(crate) use HtmlElementExt as ThisExt;

crate::element_macros::impls!(
HtmlElement : dd;
HtmlAnchorElement : a;
HtmlAreaElement : area;
HtmlAudioElement : audio;
HtmlBaseElement : base;
HtmlBodyElement : body false;
HtmlBrElement : br;
HtmlButtonElement : button;
HtmlCanvasElement : canvas;
HtmlDListElement : dl;
HtmlDataElement : data;
HtmlDataListElement : datalist;
HtmlDetailsElement : details;
HtmlDialogElement : dialog;
HtmlDivElement : div;
HtmlEmbedElement : embed;
HtmlFieldSetElement : fieldset;
HtmlFontElement : font;
HtmlFormElement : form;
HtmlFrameElement : frame;
HtmlFrameSetElement : frameset;
HtmlHeadElement : head;
HtmlHeadingElement : h1, h2, h3, h4, h5, h6;
HtmlHrElement : hr;
HtmlHtmlElement : html false;
HtmlIFrameElement : iframe;
HtmlImageElement : img;
HtmlInputElement : input;
HtmlLabelElement : label;
HtmlLegendElement : legend;
HtmlLiElement : li;
HtmlLinkElement : link;
HtmlMapElement : map;
HtmlMediaElement : ;
HtmlMenuElement : menu;
HtmlMetaElement : meta;
HtmlMeterElement : meter;
HtmlOListElement : ol;
HtmlObjectElement : object;
HtmlOptGroupElement : optgroup;
HtmlOptionElement : option;
HtmlOutputElement : output;
HtmlParagraphElement : p;
HtmlParamElement : param;
HtmlPictureElement : picture;
HtmlPreElement : pre;
HtmlProgressElement : progress;
HtmlQuoteElement : q, blockquote;
HtmlScriptElement : script;
HtmlSelectElement : select;
HtmlSlotElement : slot;
HtmlSourceElement : source;
HtmlSpanElement : span;
HtmlStyleElement : style;
HtmlTableCaptionElement : caption;
HtmlTableCellElement : td, th;
HtmlTableColElement : col, colgroup;
HtmlTableElement : table;
HtmlTableRowElement : tr;
HtmlTableSectionElement : thead, tfoot, tbody;
HtmlTemplateElement : template;
HtmlTextAreaElement : textarea;
HtmlTimeElement : time;
HtmlTitleElement : title;
HtmlTrackElement : track;
HtmlUListElement : ul;
HtmlVideoElement : video
);
