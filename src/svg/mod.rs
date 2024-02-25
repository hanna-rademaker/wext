pub trait HtmlElementExt {}

impl<T: AsRef<web_sys::HtmlElement>> HtmlElementExt for T {}

pub(crate) const NS: &'static str = "http://www.w3.org/2000/svg";

macro_rules! impls {
    ($($names:ident : $( $tagss:ident $($flagss:literal)? ),* );+) => {
        pub mod short {
            impls!(impl_short_fn -> $($names : $( $tagss $($flagss)? ),* );+);
        }
        pub mod create_ext {
            use web_sys::wasm_bindgen::JsCast;
            impls!(impl_create_ext -> $($names : $( $tagss $($flagss)? ),* );+);
        }
    };
    ($mac:ident -> $name:ident : $( $tags:ident $($flags:literal)? ),*; $($names:ident : $( $tagss:ident $($flagss:literal)? ),* );+) => {
        impls!($mac -> $name : $( $tags $($flags)? ),*);
        impls!($mac -> $($names : $( $tagss $($flagss)? ),* );+);
    };
    ($mac:ident -> $name:ident : $( $tags:ident $($flags:literal)? ),*) => {
        $mac!($name : $( $tags $($flags)? ),*);
    };
}

macro_rules! impl_create_ext {
    ($name:ident : ) => {};
    (fns -> $tag:ident $($flag:literal)?) => {
        paste::paste!{
            fn [< create_ $tag:snake >] () -> Self {
                use super::NS;
                gloo::utils::document().create_element_ns(Some(NS), stringify!($tag)).unwrap().dyn_into().unwrap()
            }
        }
    };
    (tests -> $tag:ident $($flag:literal)?) => {
        paste::paste!{
            #[wasm_bindgen_test::wasm_bindgen_test]
            fn [< test_ $tag:snake >]() {
                let _: This = This::[< create_ $tag:snake >]();
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
        paste::paste!{
            pub fn [< $tag:snake>]() -> web_sys::$name {
                 use super::create_ext::*;
                 web_sys::$name::[< create _ $tag:snake >]()
            }
            #[cfg(test)]
            #[wasm_bindgen_test::wasm_bindgen_test]
            fn [<test_ $name:snake _ $tag:snake>]() {
                [< $tag:snake >] ();
            }
        }
    };
    ($name:ident : $tag:ident $($flag:literal)?, $( $tags:ident $($flags:literal)? ),+) => {
        impl_short_fn!($name: $tag $($flag)?);
        impl_short_fn!($name: $( $tags $($flags)? ),+);
    };
}

impls!(
SvgElement : ;
SvgAnimateElement : animate;
SvgAnimateMotionElement : animateMotion;
SvgAnimateTransformElement : animateTransform;
SvgAnimationElement : ;
SvgCircleElement : circle;
SvgClipPathElement : clipPath;
SvgComponentTransferFunctionElement : ;
SvgDefsElement : defs;
SvgDescElement : desc;
SvgEllipseElement : ellipse;
SvgFilterElement : filter;
SvgForeignObjectElement : foreignObject;
SvgGeometryElement : ;
SvgGradientElement : ;
SvgGraphicsElement : ;
SvgImageElement : image;
SvgLineElement : line;
SvgLinearGradientElement : linearGradient;
SvgMarkerElement : marker;
SvgMaskElement : mask;
SvgMetadataElement : metadata;
SvgPathElement : path;
SvgPatternElement : pattern;
SvgPolygonElement : polygon;
SvgPolylineElement : polyline;
SvgRadialGradientElement : radialGradient;
SvgRectElement : rect;
SvgScriptElement : script;
SvgSetElement : set;
SvgStopElement : stop;
SvgStyleElement : style;
SvgSwitchElement : switch;
SvgSymbolElement : symbol;
SvgTextContentElement : ;
SvgTextElement : text;
SvgTextPathElement : textPath;
SvgTextPositioningElement : ;
SvgTitleElement : title;
SvgUseElement : use false; //TODO: waiting for https://github.com/dtolnay/paste/issues/74.
SvgViewElement : view;
SvgaElement : a;
SvgfeBlendElement : feBlend;
SvgfeColorMatrixElement : feColorMatrix;
SvgfeComponentTransferElement : feComponentTransfer;
SvgfeCompositeElement : feComposite;
SvgfeConvolveMatrixElement : feConvolveMatrix;
SvgfeDiffuseLightingElement : feDiffuseLighting;
SvgfeDisplacementMapElement : feDisplacementMap;
SvgfeDistantLightElement : feDistantLight;
SvgfeDropShadowElement : feDropShadow;
SvgfeFloodElement : feFlood;
SvgfeFuncAElement : feFuncA;
SvgfeFuncBElement : feFuncB;
SvgfeFuncGElement : feFuncG;
SvgfeFuncRElement : feFuncR;
SvgfeGaussianBlurElement : feGaussianBlur;
SvgfeImageElement : feImage;
SvgfeMergeElement : feMerge;
SvgfeMergeNodeElement : feMergeNode;
SvgfeMorphologyElement : feMorphology;
SvgfeOffsetElement : feOffset;
SvgfePointLightElement : fePointLight;
SvgfeSpecularLightingElement : feSpecularLighting;
SvgfeSpotLightElement : feSpotLight;
SvgfeTileElement : feTile;
SvgfeTurbulenceElement : feTurbulence;
SvggElement : g;
SvgmPathElement : mpath;
SvgsvgElement : svg;
SvgtSpanElement : tspan
);
