pub trait HtmlElementExt {}

macro_rules! impls {
    ($($names:ident : $( $tagss:ident $($flagss:literal)? ),* );+) => {
        pub mod short {
            impls!(impl_short_fn -> $($names : $( $tagss $($flagss)? ),* );+);
        }
        pub mod create_ext {
            use web_sys::wasm_bindgen::JsCast;
            impls!(impl_create_ext -> $($names : $( $tagss $($flagss)? ),* );+);
        }
        impls!(impl_parent_ext_traits -> $($names : $( $tagss $($flagss)? ),* );+);
    };
    ($mac:ident -> $name:ident : $( $tags:ident $($flags:literal)? ),*; $($names:ident : $( $tagss:ident $($flagss:literal)? ),* );+) => {
        impls!($mac -> $name : $( $tags $($flags)? ),*);
        impls!($mac -> $($names : $( $tagss $($flagss)? ),* );+);
    };
    ($mac:ident -> $name:ident : $( $tags:ident $($flags:literal)? ),*) => {
        $mac!($name : $( $tags $($flags)? ),*);
    };
}

macro_rules! impl_parent_ext_traits {
    ($name:ident : $( $tags:ident $($flags:literal)? ),*) => {
        paste::paste! {
            impl HtmlElementExt for web_sys::$name {}
            impl crate::element::ElementExt for web_sys::$name {}
            impl crate::node::NodeExt for web_sys::$name {}
        }
    };
}

macro_rules! impl_create_ext {
    ($name:ident : ) => {};
    (fns -> $tag:ident $($flag:literal)?) => {
        paste::paste!{
            fn [< create_ $tag >] () -> Self {
                gloo::utils::document().create_element(stringify!($tag)).unwrap().dyn_into().unwrap()
            }
        }
    };
    (tests -> $tag:ident $($flag:literal)?) => {
        paste::paste!{
            #[wasm_bindgen_test::wasm_bindgen_test]
            fn [< test_ $tag >]() {
                let _: This = This::[< create_ $tag >]();
            }
        }
    };
    (fns -> $tag:ident $($flag:literal)?, $( $tags:ident $($flags:literal)? ),+) => {
        impl_create_ext!(fns -> $tag $($flag)?);
        impl_create_ext!(fns -> $( $tags $($flags)? ),+);
    };
    (tests -> $tag:ident $($flag:literal)?, $( $tags:ident $($flags:literal)? ),+) => {
        impl_create_ext!(tests -> $tag $($flag)?);
        impl_create_ext!(tests -> $( $tags $($flags)? ),+);
    };
    ($name:ident : $( $tags:ident $($flags:literal)? ),+) => {
        paste::paste!{
            pub trait [< $name CreateExt >]: JsCast {
                impl_create_ext!(fns -> $( $tags $($flags)? ),+);
            }
            impl [< $name CreateExt >] for web_sys::$name {}
            #[cfg(test)]
            pub mod [< test_ $name:snake >] {
                use super::[< $name CreateExt >];
                use web_sys::$name as This;
                impl_create_ext!(tests -> $( $tags $($flags)? ),+);
            }
        }
    }
}

macro_rules! impl_short_fn {
    ($name:ident : ) => {};
    ($name:ident : $tag:ident $flag:literal) => {};
    ($name:ident : $tag:ident) => {
        pub fn $tag() -> web_sys::$name {
             use web_sys::wasm_bindgen::JsCast;
             gloo::utils::document().create_element(stringify!($tag)).unwrap().dyn_into().unwrap()
        }
        paste::paste!{
            #[cfg(test)]
            #[wasm_bindgen_test::wasm_bindgen_test]
            fn [<test_ $name:snake _ $tag>]() {
                $tag();
            }
        }
    };
    ($name:ident : $tag:ident $($flag:literal)?, $( $tags:ident $($flags:literal)? ),+) => {
        impl_short_fn!($name: $tag $($flag)?);
        impl_short_fn!($name: $( $tags $($flags)? ),+);
    };
}

impls!(
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
