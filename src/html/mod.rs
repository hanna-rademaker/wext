pub mod input;
pub trait HtmlElementExt {}

impl<T: AsRef<web_sys::HtmlElement>> HtmlElementExt for T {}

pub(crate) const NS: &'static str = "http://www.w3.org/1999/xhtml";

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
