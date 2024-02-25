macro_rules! impls {
    ($($names:ident : $( $tagss:ident $($flagss:literal)? ),* );+) => {
        pub mod short {
            crate::element_macros::impls!(impl_short_fn -> $($names : $( $tagss $($flagss)? ),* );+);
        }
        pub mod create_ext {
            use web_sys::wasm_bindgen::JsCast;
            crate::element_macros::impls!(impl_create_ext -> $($names : $( $tagss $($flagss)? ),* );+);
        }
    };
    ($mac:ident -> $name:ident : $( $tags:ident $($flags:literal)? ),*; $($names:ident : $( $tagss:ident $($flagss:literal)? ),* );+) => {
        crate::element_macros::impls!($mac -> $name : $( $tags $($flags)? ),*);
        crate::element_macros::impls!($mac -> $($names : $( $tagss $($flagss)? ),* );+);
    };
    ($mac:ident -> $name:ident : $( $tags:ident $($flags:literal)? ),*) => {
        crate::element_macros::$mac!($name : $( $tags $($flags)? ),*);
    };
}
pub(crate) use impls;

macro_rules! impl_create_ext {
    ($name:ident : ) => {};
    (fns -> $tag:ident $($flag:literal)?) => {
        paste::paste!{
            fn [< create_ $tag:snake >] () -> Self {
                use super::ThisExt;
                super::ThisBase::create(stringify!($tag)).dyn_into().unwrap()
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
        crate::element_macros::impl_create_ext!(fns -> $tag $($flag)?);
        crate::element_macros::impl_create_ext!(fns -> $( $tags $($flags)? ),+);
    };
    (tests -> $tag:ident $($flag:literal)?, $( $tags:ident $($flags:literal)? ),+) => {
        crate::element_macros::impl_create_ext!(tests -> $tag $($flag)?);
        crate::element_macros::impl_create_ext!(tests -> $( $tags $($flags)? ),+);
    };
    ($name:ident : $( $tags:ident $($flags:literal)? ),+) => {
        paste::paste!{
            pub trait [< $name CreateExt >]: JsCast {
                crate::element_macros::impl_create_ext!(fns -> $( $tags $($flags)? ),+);
            }
            impl [< $name CreateExt >] for web_sys::$name {}
            #[cfg(test)]
            pub mod [< test_ $name:snake >] {
                use super::[< $name CreateExt >];
                use web_sys::$name as This;
                crate::element_macros::impl_create_ext!(tests -> $( $tags $($flags)? ),+);
            }
        }
    }
}
pub(crate) use impl_create_ext;

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
        crate::element_macros::impl_short_fn!($name: $tag $($flag)?);
        crate::element_macros::impl_short_fn!($name: $( $tags $($flags)? ),+);
    };
}
pub(crate) use impl_short_fn;
