use crate::{
    abi::{C_ABI, SYSTEM_ABI},
    ArcDpsGen, CallbackInfo,
};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::Expr;

impl ArcDpsGen {
    /// Generates the init function contents.
    pub fn build_init(&self) -> TokenStream {
        if let Some(init) = &self.init {
            let span = syn::Error::new_spanned(init, "").span();
            quote_spanned! {span=>
                {
                    const SAFE: ::arcdps::callbacks::InitFunc = #init;
                    SAFE()
                }
            }
        } else {
            quote! { ::std::result::Result::Ok(()) }
        }
    }

    /// Generates the release function contents.
    pub fn build_release(&self) -> TokenStream {
        if let Some(release) = &self.release {
            let span = syn::Error::new_spanned(release, "").span();
            quote_spanned! {span=>
                {
                    const SAFE: ::arcdps::callbacks::ReleaseFunc = #release;
                    SAFE()
                }
            }
        } else {
            quote! {}
        }
    }

    /// Generates the update url function.
    pub fn build_update_url(&self) -> TokenStream {
        if let Some(update_url) = &self.update_url {
            let span = syn::Error::new_spanned(update_url, "").span();
            quote_spanned! {span=>
                static mut UPDATE_URL: ::std::vec::Vec<u16> = ::std::vec::Vec::new();

                #[no_mangle]
                pub unsafe extern #SYSTEM_ABI fn get_update_url() -> *const ::std::primitive::u16 {
                    const SAFE: ::arcdps::callbacks::UpdateUrlFunc = #update_url;

                    if let ::std::option::Option::Some(url) = SAFE() {
                        UPDATE_URL = ::arcdps::__macro::str_to_wide(url);
                        UPDATE_URL.as_ptr()
                    } else {
                        ::std::ptr::null()
                    }
                }
            }
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
            unsafe extern #C_ABI fn #name(
                _h_wnd: ::arcdps::__macro::HWND,
                u_msg: ::std::primitive::u32,
                w_param: ::arcdps::__macro::WPARAM,
                l_param: ::arcdps::__macro::LPARAM,
            ) -> ::std::primitive::u32 {
                const SAFE: ::arcdps::callbacks::WndProcCallback = #safe;

                match u_msg {
                    ::arcdps::__macro::WM_KEYDOWN
                    | ::arcdps::__macro::WM_KEYUP
                    | ::arcdps::__macro::WM_SYSKEYDOWN
                    | ::arcdps::__macro::WM_SYSKEYUP => {
                        let key_down = u_msg & 1 == 0;
                        let prev_key_down = (l_param.0 >> 30) & 1 == 1;

                        if SAFE(w_param.0, key_down, prev_key_down) {
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
                    unsafe extern #C_ABI fn abstract_options_windows(window_name: *const c_char) -> ::std::bool {
                        const SAFE: ::arcdps::callbacks::OptionsWindowsCallback = #safe;

                        SAFE(::arcdps::__macro::ui(), ::arcdps::__macro::str_from_cstr(window_name))
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
                    unsafe extern #C_ABI fn abstract_options_end() {
                        const SAFE: ::arcdps::callbacks::OptionsCallback = #safe;

                        SAFE(::arcdps::__macro::ui())
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
                    unsafe extern #C_ABI fn abstract_imgui(loading: ::std::primitive::u32) {
                        const SAFE: ::arcdps::callbacks::ImguiCallback = #safe;

                        SAFE(::arcdps::__macro::ui(), loading != 0)
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
            unsafe extern #C_ABI fn #name(
                event: *const ::arcdps::evtc::Event,
                src: *const ::arcdps::evtc::Agent,
                dst: *const ::arcdps::evtc::Agent,
                skill_name: *const ::arcdps::__macro::c_char,
                id: ::std::primitive::u64,
                revision: ::std::primitive::u64,
            ) {
                const SAFE: ::arcdps::callbacks::CombatCallback = #safe;

                SAFE(
                    event.as_ref(),
                    src.as_ref(),
                    dst.as_ref(),
                    ::arcdps::__macro::str_from_cstr(skill_name),
                    id,
                    revision
                )
            }
        }
    }
}
