//! Code generation for ArcDPS plugins.

mod abi;
mod callbacks;
mod export;
mod parse;

#[cfg(feature = "extras")]
mod extras;

use cfg_if::cfg_if;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{Expr, LitStr};

#[cfg(feature = "extras")]
use extras::ExtrasGen;

/// Creates plugin exports for ArcDPS.
#[proc_macro]
pub fn export(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as ArcDpsGen);

    let export = input.build_export();

    let extras_funcs = {
        cfg_if! {
            if #[cfg(feature = "extras")] {
                let name = input.gen_name_cstr();
                input.extras.build(name)
            } else {
                quote! {}
            }
        }
    };

    let result = quote! {
        mod __arcdps_gen_export {
            use super::*;

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
    update_url: Option<Expr>,

    raw_combat: Option<Expr>,
    combat: Option<Expr>,

    raw_combat_local: Option<Expr>,
    combat_local: Option<Expr>,

    raw_imgui: Option<Expr>,
    imgui: Option<Expr>,

    raw_options_end: Option<Expr>,
    options_end: Option<Expr>,

    raw_options_windows: Option<Expr>,
    options_windows: Option<Expr>,

    raw_wnd_filter: Option<Expr>,
    wnd_filter: Option<Expr>,

    raw_wnd_nofilter: Option<Expr>,
    wnd_nofilter: Option<Expr>,

    #[cfg(feature = "extras")]
    extras: ExtrasGen,
}

impl Default for ArcDpsGen {
    fn default() -> Self {
        Self {
            name: None,
            sig: Expr::Verbatim(TokenStream::new()),
            init: None,
            release: None,
            update_url: None,

            raw_combat: None,
            combat: None,

            raw_combat_local: None,
            combat_local: None,

            raw_imgui: None,
            imgui: None,

            raw_options_end: None,
            options_end: None,

            raw_options_windows: None,
            options_windows: None,

            raw_wnd_filter: None,
            wnd_filter: None,

            raw_wnd_nofilter: None,
            wnd_nofilter: None,

            #[cfg(feature = "extras")]
            extras: ExtrasGen::default(),
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
            .unwrap_or_else(|| CallbackInfo::new(quote! {}, quote! { ::std::option::Option::None }))
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
            let span = syn::Error::new_spanned(raw, "").span();
            let value = quote_spanned!(span=> ::std::option::Option::Some((#raw) as _) );

            Some(CallbackInfo::new(quote! {}, value))
        } else if let Some(safe) = safe {
            let span = syn::Error::new_spanned(safe, "").span();
            let func = wrapper(safe, span);
            let value = quote_spanned!(span=> ::std::option::Option::Some(self::#name as _) );

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
