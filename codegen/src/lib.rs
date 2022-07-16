mod callbacks;
mod export;
mod parse;

#[cfg(feature = "extras")]
mod extras;

use cfg_if::cfg_if;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
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
            use super::*;
            use ::arcdps::__macro::prelude::*;

            #export

            #extras_funcs
        }
    };

    result.into()
}

/// Helper to generate code.
///
/// Holds information about macro input.
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
    raw_extras_language_changed: Option<Expr>,

    #[cfg(feature = "extras")]
    extras_init: Option<Expr>,

    #[cfg(feature = "extras")]
    extras_squad_update: Option<Expr>,

    #[cfg(feature = "extras")]
    extras_language_changed: Option<Expr>,
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
            raw_extras_language_changed: None,

            #[cfg(feature = "extras")]
            extras_init: None,

            #[cfg(feature = "extras")]
            extras_squad_update: None,

            #[cfg(feature = "extras")]
            extras_language_changed: None,
        }
    }
}

/// Helper to represent callback information.
pub(crate) struct CallbackInfo {
    pub function: TokenStream,
    pub value: TokenStream,
}

impl CallbackInfo {
    /// Creates a new callback info from token streams.
    pub fn new(function: TokenStream, value: TokenStream) -> Self {
        Self { function, value }
    }

    /// Helper to build a callback.
    ///
    /// `raw` is the value of the raw callback if passed to the macro.
    /// `safe` is the value of the safe callback if passed to the macro.
    /// `name` is the name of the abstract wrapper function for the safe version.
    /// `wrapper` generates the abstract wrapper if needed.
    pub fn build(
        raw: Option<&Expr>,
        safe: Option<&Expr>,
        name: TokenStream,
        wrapper: impl FnOnce(&Expr, Span) -> TokenStream,
    ) -> CallbackInfo {
        Self::build_optional(raw, safe, name, wrapper)
            .unwrap_or_else(|| CallbackInfo::new(quote! {}, quote! { None }))
    }

    /// Helper to build an optional callback.
    ///
    /// See `build` for more info.
    pub fn build_optional(
        raw: Option<&Expr>,
        safe: Option<&Expr>,
        name: TokenStream,
        wrapper: impl FnOnce(&Expr, Span) -> TokenStream,
    ) -> Option<CallbackInfo> {
        if let Some(raw) = raw {
            let span = syn::Error::new_spanned(&raw, "").span();
            let value = quote_spanned!(span=> Some((#raw) as _) );

            Some(CallbackInfo::new(quote! {}, value))
        } else if let Some(safe) = safe {
            let span = syn::Error::new_spanned(&safe, "").span();
            let func = wrapper(safe, span);
            let value = quote_spanned!(span=> Some(self::#name as _) );

            Some(CallbackInfo::new(func, value))
        } else {
            None
        }
    }

    /// Returns the callback info as tuple.
    pub fn into_tuple(self) -> (TokenStream, TokenStream) {
        (self.function, self.value)
    }
}
