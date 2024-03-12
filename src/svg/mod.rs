pub trait SvgElementExt: AsRef<web_sys::SvgElement> + Clone {
    fn create(tag: impl AsRef<str>) -> Self
    where
        Self: Sized + web_sys::wasm_bindgen::JsCast,
    {
        use web_sys::wasm_bindgen::JsCast;
        gloo::utils::document()
            .create_element_ns(Some("http://www.w3.org/2000/svg"), tag.as_ref())
            .unwrap()
            .dyn_into()
            .unwrap()
    }
    fn css(&self, property: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.as_ref().style().set_property(property.as_ref(), value.as_ref()).unwrap();
        self.clone()
    }
}

impl<T: AsRef<web_sys::SvgElement> + Clone> SvgElementExt for T {}

pub(crate) type ThisBase = web_sys::SvgElement;
pub(crate) use SvgElementExt as ThisExt;

crate::element_macros::impls!(
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
