use crate::ArcDpsGen;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::Expr;

impl ArcDpsGen {
    pub fn build_init(&self) -> TokenStream {
        if let Some(init) = &self.init {
            let span = syn::Error::new_spanned(&init, "").span();
            quote_spanned!(span=> (super::#init as InitFunc)())
        } else {
            quote! { Ok(()) }
        }
    }

    pub fn build_release(&self) -> TokenStream {
        if let Some(release) = &self.release {
            let span = syn::Error::new_spanned(&release, "").span();
            quote_spanned!(span=> (super::#release as ReleaseFunc)();)
        } else {
            quote! {}
        }
    }

    pub fn build_wnd_filter(&self) -> (TokenStream, TokenStream) {
        Self::build_wnd(
            self.raw_wnd_filter.as_ref(),
            self.wnd_filter.as_ref(),
            quote! { abstract_wnd_filter },
        )
    }

    pub fn build_wnd_nofilter(&self) -> (TokenStream, TokenStream) {
        Self::build_wnd(
            self.raw_wnd_nofilter.as_ref(),
            self.wnd_nofilter.as_ref(),
            quote! { abstract_wnd_nofilter },
        )
    }

    pub fn build_wnd(
        raw_wnd_filter: Option<&Expr>,
        wnd_filter: Option<&Expr>,
        func_name: TokenStream,
    ) -> (TokenStream, TokenStream) {
        match (raw_wnd_filter, wnd_filter) {
            (Some(raw), _) => {
                let span = syn::Error::new_spanned(&raw, "").span();
                let name = quote_spanned!(span=> Some(super::#raw as _) );

                (quote! {}, name)
            }
            (_, Some(safe)) => {
                let span = syn::Error::new_spanned(&safe, "").span();
                let wrapper = quote_spanned! {span=>
                    unsafe extern "C" fn #func_name(_h_wnd: *mut c_void, u_msg: u32, w_param: WPARAM, l_param: LPARAM) -> u32 {
                        let _ = super::#safe as WndProcCallback;

                        match u_msg {
                            WM_KEYDOWN | WM_KEYUP | WM_SYSKEYDOWN | WM_SYSKEYUP => {
                                let key_down = u_msg & 1 == 0;
                                let prev_key_down = (l_param.0 >> 30) & 1 == 1;

                                if super::#safe(w_param.0, key_down, prev_key_down) {
                                    u_msg
                                } else {
                                    0
                                }
                            },
                            _ => u_msg,
                        }
                    }
                };
                let name = quote_spanned!(span=> Some(self::#func_name as _) );

                (wrapper, name)
            }
            _ => (quote! {}, quote! { None }),
        }
    }

    pub fn build_options_windows(&self) -> (TokenStream, TokenStream) {
        match (&self.raw_options_windows, &self.options_windows) {
            (Some(raw), _) => {
                let span = syn::Error::new_spanned(&raw, "").span();
                let name = quote_spanned!(span=> Some(super::#raw as _) );

                (quote! {}, name)
            }
            (_, Some(safe)) => {
                let span = syn::Error::new_spanned(&safe, "").span();
                let wrapper = quote_spanned! {span =>
                    unsafe extern "C" fn abstract_options_windows(window_name: *mut c_char) -> bool {
                        let _ = super::#safe as OptionsWindowsCallback;

                        super::#safe(__ui(), str_from_cstr(window_name))
                    }
                };
                let name = quote_spanned!(span=> Some(self::abstract_options_windows as _) );

                (wrapper, name)
            }
            _ => (quote! {}, quote! { None }),
        }
    }

    pub fn build_options_end(&self) -> (TokenStream, TokenStream) {
        match (&self.raw_options_end, &self.options_end) {
            (Some(raw), _) => {
                let span = syn::Error::new_spanned(&raw, "").span();
                let name = quote_spanned!(span=> Some(super::#raw as _) );

                (quote! {}, name)
            }
            (_, Some(safe)) => {
                let span = syn::Error::new_spanned(&safe, "").span();
                let wrapper = quote_spanned! {span =>
                    unsafe extern "C" fn abstract_options_end() {
                        let _ = super::#safe as OptionsCallback;

                        super::#safe(__ui())
                    }
                };
                let name = quote_spanned!(span=> Some(self::abstract_options_end as _) );

                (wrapper, name)
            }
            _ => (quote! {}, quote! { None }),
        }
    }

    pub fn build_imgui(&self) -> (TokenStream, TokenStream) {
        match (&self.raw_imgui, &self.imgui) {
            (Some(raw), _) => {
                let span = syn::Error::new_spanned(&raw, "").span();
                let name = quote_spanned!(span=> Some(super::#raw as _) );

                (quote! {}, name)
            }
            (_, Some(safe)) => {
                let span = syn::Error::new_spanned(&safe, "").span();
                let wrapper = quote_spanned! {span =>
                    unsafe extern "C" fn abstract_imgui(loading: u32) {
                        let _ = super::#safe as ImguiCallback;

                        super::#safe(__ui(), loading != 0)
                    }
                };
                let name = quote_spanned!(span=> Some(self::abstract_imgui as _) );

                (wrapper, name)
            }
            _ => (quote! {}, quote! { None }),
        }
    }

    pub fn build_combat(&self) -> (TokenStream, TokenStream) {
        Self::build_combat_helper(
            self.raw_combat.as_ref(),
            self.combat.as_ref(),
            quote! { abstract_combat },
        )
    }

    pub fn build_combat_local(&self) -> (TokenStream, TokenStream) {
        Self::build_combat_helper(
            self.raw_combat_local.as_ref(),
            self.combat_local.as_ref(),
            quote! { abstract_combat_local },
        )
    }

    pub fn build_combat_helper(
        raw_combat: Option<&Expr>,
        combat: Option<&Expr>,
        func_name: TokenStream,
    ) -> (TokenStream, TokenStream) {
        match (raw_combat, combat) {
            (Some(raw), _) => {
                let span = syn::Error::new_spanned(&raw, "").span();
                let name = quote_spanned!(span=> Some(super::#raw as _) );

                (quote! {}, name)
            }
            (_, Some(safe)) => {
                let span = syn::Error::new_spanned(&safe, "").span();
                let wrapper = quote_spanned! {span =>
                    unsafe extern "C" fn #func_name(
                            event: Option<&::arcdps::api::RawCombatEvent>,
                            src: Option<&::arcdps::api::RawAgent>,
                            dst: Option<&::arcdps::api::RawAgent>,
                            skill_name: *mut c_char,
                            id: u64,
                            revision: u64,
                        ) {
                            let _ = super::#safe as CombatCallback;

                            super::#safe(
                                event.map(Into::into),
                                src.map(Into::into),
                                dst.map(Into::into),
                                str_from_cstr(skill_name),
                                id,
                                revision
                            )
                    }
                };
                let name = quote_spanned!(span=> Some(self::#func_name as _) );

                (wrapper, name)
            }
            _ => (quote! {}, quote! { None }),
        }
    }
}
