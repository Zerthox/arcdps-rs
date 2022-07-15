mod callbacks;
mod export;
mod parse;

#[cfg(feature = "extras")]
mod extras;

use cfg_if::cfg_if;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, LitStr};

/// Creates exports for ArcDPS.
#[proc_macro]
pub fn export(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as ArcDpsGen);

    let export = input.build_export();

    let extras_funcs = {
        cfg_if! {
            if #[cfg(feature = "extras")] {
                input.build_extras()
            } else {
                quote! {}
            }
        }
    };

    let result = quote! {
        mod __arcdps_gen_export {
            use ::arcdps::__macro::*;

            #export

            #extras_funcs
        }
    };

    result.into()
}

// TODO: absolute paths like `crate::init`? closures?
pub(crate) struct ArcDpsGen {
    name: Option<LitStr>,
    sig: Expr,
    init: Option<Expr>,
    release: Option<Expr>,

    raw_combat: Option<Expr>,
    raw_combat_local: Option<Expr>,
    raw_imgui: Option<Expr>,
    raw_options_end: Option<Expr>,
    raw_options_windows: Option<Expr>,
    raw_wnd_filter: Option<Expr>,
    raw_wnd_nofilter: Option<Expr>,

    combat: Option<Expr>,
    combat_local: Option<Expr>,
    imgui: Option<Expr>,
    options_end: Option<Expr>,
    options_windows: Option<Expr>,
    wnd_filter: Option<Expr>,
    wnd_nofilter: Option<Expr>,

    #[cfg(feature = "extras")]
    raw_extras_init: Option<Expr>,

    #[cfg(feature = "extras")]
    raw_extras_squad_update: Option<Expr>,

    #[cfg(feature = "extras")]
    extras_init: Option<Expr>,

    #[cfg(feature = "extras")]
    extras_squad_update: Option<Expr>,
}

impl Default for ArcDpsGen {
    fn default() -> Self {
        Self {
            name: None,
            sig: Expr::Verbatim(TokenStream::new()),
            init: None,
            release: None,

            raw_combat: None,
            raw_combat_local: None,
            raw_imgui: None,
            raw_options_end: None,
            raw_options_windows: None,
            raw_wnd_filter: None,
            raw_wnd_nofilter: None,

            combat: None,
            combat_local: None,
            imgui: None,
            options_end: None,
            options_windows: None,
            wnd_filter: None,
            wnd_nofilter: None,

            #[cfg(feature = "extras")]
            raw_extras_init: None,

            #[cfg(feature = "extras")]
            raw_extras_squad_update: None,

            #[cfg(feature = "extras")]
            extras_init: None,

            #[cfg(feature = "extras")]
            extras_squad_update: None,
        }
    }
}
