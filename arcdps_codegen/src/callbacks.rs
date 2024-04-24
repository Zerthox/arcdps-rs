use crate::{
    abi::{C_ABI, SYSTEM_ABI},
    ArcDpsGen, CallbackInfo,
};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{Expr, Ident};

impl ArcDpsGen {
    /// Generates the init function.
    pub fn build_init(&self) -> CallbackInfo {
        if let Some(init) = &self.init {
            let span = syn::Error::new_spanned(init, "").span();
            CallbackInfo::new(
                quote_spanned! {span=>
                    const __INIT: ::arcdps::callbacks::InitFunc = #init;
                },
                quote_spanned! {span=>
                    self::__INIT()
                },
            )
        } else {
            CallbackInfo::new(quote! {}, quote! { ::std::result::Result::Ok(()) })
        }
    }

    /// Generates the release function.
    pub fn build_release(&self) -> CallbackInfo {
        if let Some(release) = &self.release {
            let span = syn::Error::new_spanned(release, "").span();
            CallbackInfo::new(
                quote_spanned! {span=>
                    const __RELEASE: ::arcdps::callbacks::ReleaseFunc = #release;
                },
                quote_spanned! {span=>
                    self::__RELEASE()
                },
            )
        } else {
            CallbackInfo::empty()
        }
    }

    /// Generates the update url function.
    pub fn build_update_url(&self) -> TokenStream {
        if let Some(update_url) = &self.update_url {
            let span = syn::Error::new_spanned(update_url, "").span();
            quote_spanned! {span=>
                const __UPDATE_URL: ::arcdps::callbacks::UpdateUrlFunc = #update_url;

                #[no_mangle]
                pub unsafe extern #SYSTEM_ABI fn get_update_url() -> *const ::std::primitive::u16 {
                    static mut URL: ::std::vec::Vec<u16> = ::std::vec::Vec::new();

                    if let ::std::option::Option::Some(url) = self::__UPDATE_URL() {
                        URL = ::arcdps::__macro::str_to_wide(url);
                        URL.as_ptr()
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
        CallbackInfo::build(
            self.raw_wnd_filter.as_ref(),
            self.wnd_filter.as_ref(),
            "__wnd_filter",
            |name, safe, span| Self::wnd_wrapper("__WND_FILTER", name, safe, span),
        )
    }

    /// Generates the wnd nofilter callback.
    pub fn build_wnd_nofilter(&self) -> CallbackInfo {
        CallbackInfo::build(
            self.raw_wnd_nofilter.as_ref(),
            self.wnd_nofilter.as_ref(),
            "__wnd_nofilter",
            |name, safe, span| Self::wnd_wrapper("__WND_NOFILTER", name, safe, span),
        )
    }

    /// Helper to generate a wnd callback wrapper.
    fn wnd_wrapper(name: &str, func_name: &Ident, safe: &Expr, span: Span) -> TokenStream {
        let name = Ident::new(name, Span::call_site());
        quote_spanned! {span=>
            const #name: ::arcdps::callbacks::WndProcCallback = #safe;

            unsafe extern #C_ABI fn #func_name(
                _h_wnd: ::arcdps::__macro::HWND,
                u_msg: ::std::primitive::u32,
                w_param: ::arcdps::__macro::WPARAM,
                l_param: ::arcdps::__macro::LPARAM,
            ) -> ::std::primitive::u32 {
                match u_msg {
                    ::arcdps::__macro::WM_KEYDOWN
                    | ::arcdps::__macro::WM_KEYUP
                    | ::arcdps::__macro::WM_SYSKEYDOWN
                    | ::arcdps::__macro::WM_SYSKEYUP => {
                        let key_down = u_msg & 1 == 0;
                        let prev_key_down = (l_param.0 >> 30) & 1 == 1;

                        if self::#name(w_param.0, key_down, prev_key_down) {
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
            "__options_windows",
            |name, safe, span| {
                quote_spanned! {span=>
                    const __OPTIONS_WINDOWS: ::arcdps::callbacks::OptionsWindowsCallback = #safe;

                    unsafe extern #C_ABI fn #name(window_name: *const ::arcdps::__macro::c_char) -> ::std::primitive::bool {
                        self::__OPTIONS_WINDOWS(::arcdps::__macro::ui(), ::arcdps::__macro::str_from_cstr(window_name))
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
            "__options_end",
            |name, safe, span| {
                quote_spanned! {span=>
                    const __OPTIONS_END: ::arcdps::callbacks::OptionsCallback = #safe;

                    unsafe extern #C_ABI fn #name() {
                        self::__OPTIONS_END(::arcdps::__macro::ui())
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
            "__imgui",
            |name, safe, span| {
                quote_spanned! {span=>
                    const __IMGUI: ::arcdps::callbacks::ImguiCallback = #safe;

                    unsafe extern #C_ABI fn #name(loading: ::std::primitive::u32) {
                        self::__IMGUI(::arcdps::__macro::ui(), loading != 0)
                    }
                }
            },
        )
    }

    /// Generates the combat callback.
    pub fn build_combat(&self) -> CallbackInfo {
        CallbackInfo::build(
            self.raw_combat.as_ref(),
            self.combat.as_ref(),
            "__combat",
            |name, safe, span| Self::combat_wrapper("__COMBAT", name, safe, span),
        )
    }

    /// Generates the combat local callback.
    pub fn build_combat_local(&self) -> CallbackInfo {
        CallbackInfo::build(
            self.raw_combat_local.as_ref(),
            self.combat_local.as_ref(),
            "__combat_local",
            |name, safe, span| Self::combat_wrapper("__COMBAT_LOCAL", name, safe, span),
        )
    }

    /// Helper to generate a combat callback wrapper.
    fn combat_wrapper(name: &str, func_name: &Ident, safe: &Expr, span: Span) -> TokenStream {
        let name = Ident::new(name, Span::call_site());
        quote_spanned! {span=>
            const #name: ::arcdps::callbacks::CombatCallback = #safe;

            unsafe extern #C_ABI fn #func_name(
                event: *const ::arcdps::evtc::Event,
                src: *const ::arcdps::evtc::Agent,
                dst: *const ::arcdps::evtc::Agent,
                skill_name: *const ::arcdps::__macro::c_char,
                id: ::std::primitive::u64,
                revision: ::std::primitive::u64,
            ) {

                self::#name(
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
