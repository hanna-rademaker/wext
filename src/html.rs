use gloo::utils::document;
use web_sys::HtmlElement;
use web_sys::wasm_bindgen::JsCast;
use crate::element::ElementExt;

pub trait HtmlElementExt: AsRef<HtmlElement> + Sized {
    const DEFAULT_TAG: &'static str;
}

impl<T> ElementExt for T
    where
        T: HtmlElementExt,
{
    fn create() -> Self
        where
            Self: JsCast + Sized + Clone,
    {
        document().create_element(Self::DEFAULT_TAG).unwrap().dyn_into().unwrap()
    }
}

macro_rules! html_impl {
    (noshort $name: ident, $($tags:ident),+) => {
        html_impl_element!( $name, $($tags),+ );
    };
    ($name: ident, $($tags:ident),+) => {
        html_impl!(noshort $name, $($tags),+ );
        html_impl_tag!( $name, $($tags),+);
    };
}

macro_rules! html_impl_tag {
    ($name: ident, $tag:ident) => {
        paste::paste!{
            pub fn [< $tag:snake >]() -> web_sys::$name {
                document().create_element(stringify!($tag)).unwrap().dyn_into().unwrap()
            }
            #[cfg(test)]
            pub mod [<test_ $tag:snake>] {
                use wasm_bindgen_test::*;

                #[wasm_bindgen_test]
                fn [<test_ $tag:snake>]() {
                    super::[< $tag:snake >]();
                }
            }
        }
    };
    ($name: ident, $tag:ident, $($tags:ident),+) => {
        html_impl_tag!( $name, $tag );
        html_impl_tag!( $name, $($tags),+ );
    };
}

macro_rules! html_impl_element {
    ($name:ident, $tag:ident) => {
        paste::paste! {
            impl HtmlElementExt for web_sys::$name {
                const DEFAULT_TAG: &'static str = stringify!($tag);
            }
            #[cfg(test)]
            pub mod [<test_ $name:snake>] {
                use crate::prelude::*;
                use wasm_bindgen_test::*;

                #[wasm_bindgen_test]
                fn test_create() {
                    web_sys::$name::create();
                }
            }
        }
    };
    ($name:ident, $tag:ident, $($tags:ident),+) => {
        html_impl_element!($name, $tag);
    };
}

html_impl!(HtmlElement, dd);

html_impl!(HtmlAnchorElement, a);
html_impl!(HtmlAreaElement, area);
html_impl!(HtmlAudioElement, audio);
html_impl!(HtmlBaseElement, base);
html_impl!(noshort HtmlBodyElement, body);
html_impl!(HtmlBrElement, br);
html_impl!(HtmlButtonElement, button);
html_impl!(HtmlCanvasElement, canvas);
html_impl!(HtmlDListElement, dl);
html_impl!(HtmlDataElement, data);
html_impl!(HtmlDataListElement, datalist);
html_impl!(HtmlDetailsElement, details);
html_impl!(HtmlDialogElement, dialog);
html_impl!(HtmlDivElement, div);
html_impl!(HtmlEmbedElement, embed);
html_impl!(HtmlFieldSetElement, fieldset);
html_impl!(HtmlFontElement, font);
html_impl!(HtmlFormElement, form);
html_impl!(HtmlFrameElement, frame);
html_impl!(HtmlFrameSetElement, frameset);
html_impl!(HtmlHeadElement, head);
html_impl!(HtmlHeadingElement, h1, h2, h3, h4, h5, h6);
html_impl!(HtmlHrElement, hr);
html_impl!(noshort HtmlHtmlElement, html);
html_impl!(HtmlIFrameElement, iframe);
html_impl!(HtmlImageElement, img);
html_impl!(HtmlInputElement, input);
html_impl!(HtmlLabelElement, label);
html_impl!(HtmlLegendElement, legend);
html_impl!(HtmlLiElement, li);
html_impl!(HtmlLinkElement, link);
html_impl!(HtmlMapElement, map);
html_impl!(noshort HtmlMediaElement, video);
html_impl!(HtmlMenuElement, menu);
html_impl!(HtmlMetaElement, meta);
html_impl!(HtmlMeterElement, meter);
html_impl!(HtmlOListElement, ol);
html_impl!(HtmlObjectElement, object);
html_impl!(HtmlOptGroupElement, optgroup);
html_impl!(HtmlOptionElement, option);
html_impl!(HtmlOutputElement, output);
html_impl!(HtmlParagraphElement, p);
html_impl!(HtmlParamElement, param);
html_impl!(HtmlPictureElement, picture);
html_impl!(HtmlPreElement, pre);
html_impl!(HtmlProgressElement, progress);
html_impl!(HtmlQuoteElement, q, blockquote);
html_impl!(HtmlScriptElement, script);
html_impl!(HtmlSelectElement, select);
html_impl!(HtmlSlotElement, slot);
html_impl!(HtmlSourceElement, source);
html_impl!(HtmlSpanElement, span);
html_impl!(HtmlStyleElement, style);
html_impl!(HtmlTableCaptionElement, caption);
html_impl!(HtmlTableCellElement, td, th);
html_impl!(HtmlTableColElement, col, colgroup);
html_impl!(HtmlTableElement, table);
html_impl!(HtmlTableRowElement, tr);
html_impl!(HtmlTableSectionElement, thead, tfoot, tbody);
html_impl!(HtmlTemplateElement, template);
html_impl!(HtmlTextAreaElement, textarea);
html_impl!(HtmlTimeElement, time);
html_impl!(HtmlTitleElement, title);
html_impl!(HtmlTrackElement, track);
html_impl!(HtmlUListElement, ul);
html_impl!(HtmlVideoElement, video);