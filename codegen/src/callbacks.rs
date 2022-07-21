use crate::{ArcDpsGen, CallbackInfo};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::Expr;

impl ArcDpsGen {
    /// Generates the init function.
    pub fn build_init(&self) -> TokenStream {
        if let Some(init) = &self.init {
            let span = syn::Error::new_spanned(&init, "").span();
            quote_spanned!(span=> ((#init) as InitFunc)())
        } else {
            quote! { Ok(()) }
        }
    }

    /// Generates the release function.
    pub fn build_release(&self) -> TokenStream {
        if let Some(release) = &self.release {
            let span = syn::Error::new_spanned(&release, "").span();
            quote_spanned!(span=> ((#release) as ReleaseFunc)();)
        } else {
            quote! {}
        }
    }

    /// Generates the wnd filter callback.
    pub fn build_wnd_filter(&self) -> CallbackInfo {
        let name = quote! { abstract_wnd_filter };
        CallbackInfo::build(
            self.raw_wnd_filter.as_ref(),
            self.wnd_filter.as_ref(),
            name.clone(),
            |safe, span| Self::wnd_wrapper(name, safe, span),
        )
    }

    /// Generates the wnd nofilter callback.
    pub fn build_wnd_nofilter(&self) -> CallbackInfo {
        let name = quote! { abstract_wnd_nofilter };
        CallbackInfo::build(
            self.raw_wnd_nofilter.as_ref(),
            self.wnd_nofilter.as_ref(),
            name.clone(),
            |safe, span| Self::wnd_wrapper(name, safe, span),
        )
    }

    /// Helper to generate a wnd callback wrapper.
    fn wnd_wrapper(name: TokenStream, safe: &Expr, span: Span) -> TokenStream {
        quote_spanned! {span=>
            unsafe extern "C" fn #name(
                _h_wnd: HWND,
                u_msg: u32,
                w_param: WPARAM,
                l_param: LPARAM,
            ) -> u32 {
                let safe = (#safe) as WndProcCallback;

                match u_msg {
                    WM_KEYDOWN | WM_KEYUP | WM_SYSKEYDOWN | WM_SYSKEYUP => {
                        let key_down = u_msg & 1 == 0;
                        let prev_key_down = (l_param.0 >> 30) & 1 == 1;

                        if safe(w_param.0, key_down, prev_key_down) {
                            u_msg
                        } else {
                            0
                        }
                    },
                    _ => u_msg,
                }
            }
        }
    }

    /// Generates the options windows callback.
    pub fn build_options_windows(&self) -> CallbackInfo {
        CallbackInfo::build(
            self.raw_options_windows.as_ref(),
            self.options_windows.as_ref(),
            quote! { abstract_options_windows },
            |safe, span| {
                quote_spanned! {span=>
                    unsafe extern "C" fn abstract_options_windows(window_name: *mut c_char) -> bool {
                        let safe = (#safe) as OptionsWindowsCallback;
                        safe(::arcdps::__macro::ui(), ::arcdps::__macro::str_from_cstr(window_name))
                    }
                }
            },
        )
    }

    /// Generates the options end callback.
    pub fn build_options_end(&self) -> CallbackInfo {
        CallbackInfo::build(
            self.raw_options_end.as_ref(),
            self.options_end.as_ref(),
            quote! { abstract_options_end },
            |safe, span| {
                quote_spanned! {span=>
                    unsafe extern "C" fn abstract_options_end() {
                        let safe = (#safe) as OptionsCallback;
                        safe(::arcdps::__macro::ui())
                    }
                }
            },
        )
    }

    /// Generates the imgui callback.
    pub fn build_imgui(&self) -> CallbackInfo {
        CallbackInfo::build(
            self.raw_imgui.as_ref(),
            self.imgui.as_ref(),
            quote! { abstract_imgui },
            |safe, span| {
                quote_spanned! {span=>
                    unsafe extern "C" fn abstract_imgui(loading: u32) {
                        let safe = (#safe) as ImguiCallback;
                        safe(::arcdps::__macro::ui(), loading != 0)
                    }
                }
            },
        )
    }

    /// Generates the combat callback.
    pub fn build_combat(&self) -> CallbackInfo {
        let name = quote! { abstract_combat };
        CallbackInfo::build(
            self.raw_combat.as_ref(),
            self.combat.as_ref(),
            name.clone(),
            |safe, span| Self::combat_wrapper(name, safe, span),
        )
    }

    /// Generates the combat local callback.
    pub fn build_combat_local(&self) -> CallbackInfo {
        let name = quote! { abstract_combat_local };
        CallbackInfo::build(
            self.raw_combat_local.as_ref(),
            self.combat_local.as_ref(),
            name.clone(),
            |safe, span| Self::combat_wrapper(name, safe, span),
        )
    }

    /// Helper to generate a combat callback wrapper.
    fn combat_wrapper(name: TokenStream, safe: &Expr, span: Span) -> TokenStream {
        quote_spanned! {span=>
            unsafe extern "C" fn #name(
                event: Option<&::arcdps::api::RawCombatEvent>,
                src: Option<&::arcdps::api::RawAgent>,
                dst: Option<&::arcdps::api::RawAgent>,
                skill_name: *mut c_char,
                id: u64,
                revision: u64,
            ) {
                let safe = (#safe) as CombatCallback;

                safe(
                    event.clone().map(Into::into),
                    src.clone().map(Into::into),
                    dst.clone().map(Into::into),
                    ::arcdps::__macro::str_from_cstr(skill_name),
                    id,
                    revision
                )
            }
        }
    }
}
