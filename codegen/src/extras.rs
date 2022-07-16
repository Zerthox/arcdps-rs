use crate::ArcDpsGen;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, LitStr};

impl ArcDpsGen {
    pub fn build_extras(&self) -> TokenStream {
        let name = self.gen_name();

        let (squad_update_func, squad_update_name) = self.build_extras_squad_update();
        let (language_changed_func, language_changed_name) = self.build_extras_language_changed();
        let init_func = self.build_extras_init(&name, squad_update_name, language_changed_name);

        quote! {
            #init_func
            #squad_update_func
            #language_changed_func
        }
    }

    fn build_extras_init(
        &self,
        name: &LitStr,
        squad_update: Option<TokenStream>,
        language_changed: Option<TokenStream>,
    ) -> TokenStream {
        let has_callback = squad_update.is_some() || language_changed.is_some();
        let squad_callback = squad_update.unwrap_or(quote! { None });
        let lang_callback = language_changed.unwrap_or(quote! { None });

        // we only subscribe if compat check passes
        // info may still be read for safe version
        let subscribe = quote! {
            if addon.check_compat() {
                sub.subscribe(#name, #squad_callback, #lang_callback);
            }
        };

        let content = match (&self.raw_extras_init, &self.extras_init) {
            (Some(raw), _) => {
                let span = syn::Error::new_spanned(&raw, "").span();
                quote_spanned! {span=>
                    let raw = (#raw) as RawExtrasSubscriberInit;

                    raw(addon, sub)
                }
            }
            (_, Some(safe)) => {
                let span = syn::Error::new_spanned(&safe, "").span();
                quote_spanned! {span=>
                    let safe = (#safe) as ExtrasInitFunc;

                    #subscribe

                    let user = ::arcdps::__macro::str_from_cstr(addon.self_account_name as _)
                        .map(|n| n.trim_start_matches(':'));

                    safe(addon.into(), user);
                }
            }
            _ if has_callback => quote! {
                    #subscribe
            },
            _ => return quote! {},
        };

        quote_spanned! {content.span()=>
            #[no_mangle]
            unsafe extern "system" fn arcdps_unofficial_extras_subscriber_init(
                addon: &::arcdps::extras::RawExtrasAddonInfo,
                sub: &mut ::arcdps::extras::ExtrasSubscriberInfo
            ) {
                #content
            }
        }
    }

    fn build_extras_squad_update(&self) -> (TokenStream, Option<TokenStream>) {
        match (&self.raw_extras_squad_update, &self.extras_squad_update) {
            (Some(raw), _) => {
                let span = syn::Error::new_spanned(&raw, "").span();
                let name = quote_spanned!(span=> Some((#raw) as _) );

                (quote! {}, Some(name))
            }
            (_, Some(safe)) => {
                let span = syn::Error::new_spanned(&safe, "").span();
                let wrapper = quote_spanned! {span=>
                    unsafe extern "C" fn abstract_extras_squad_update(
                        users: *const ::arcdps::extras::RawUserInfo,
                        count: u64
                    ) {
                        let safe = (#safe) as ExtrasSquadUpdateCallback;
                        safe(::arcdps::extras::to_user_info_iter(users, count))
                    }
                };
                let name = quote_spanned!(span=> Some(self::abstract_extras_squad_update as _) );

                (wrapper, Some(name))
            }
            _ => (quote! {}, None),
        }
    }

    fn build_extras_language_changed(&self) -> (TokenStream, Option<TokenStream>) {
        match (
            &self.raw_extras_language_changed,
            &self.extras_language_changed,
        ) {
            (Some(raw), _) => {
                let span = syn::Error::new_spanned(&raw, "").span();
                let name = quote_spanned!(span=> Some((#raw) as _) );

                (quote! {}, Some(name))
            }
            (_, Some(safe)) => {
                let span = syn::Error::new_spanned(&safe, "").span();
                let wrapper = quote_spanned! {span=>
                    unsafe extern "C" fn abstract_extras_language_changed(language: ::arcdps::api::Language) {
                        let safe = (#safe) as ExtrasLanguageChangedCallback;
                        safe(language)
                    }
                };
                let name =
                    quote_spanned!(span=> Some(self::abstract_extras_language_changed as _) );

                (wrapper, Some(name))
            }
            _ => (quote! {}, None),
        }
    }
}
