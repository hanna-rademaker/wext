pub trait HtmlElementExt {}

impl<T: AsRef<web_sys::HtmlElement>> HtmlElementExt for T {}

pub(crate) const NS: &'static str = "http://www.w3.org/2000/svg";

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
