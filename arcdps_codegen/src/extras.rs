use crate::{
    abi::{C_ABI, SYSTEM_ABI},
    CallbackInfo,
};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Expr, LitStr};

#[derive(Default)]
pub(crate) struct ExtrasGen {
    pub raw_extras_init: Option<Expr>,
    pub extras_init: Option<Expr>,

    pub raw_extras_squad_update: Option<Expr>,
    pub extras_squad_update: Option<Expr>,

    pub raw_extras_language_changed: Option<Expr>,
    pub extras_language_changed: Option<Expr>,

    pub raw_extras_keybind_changed: Option<Expr>,
    pub extras_keybind_changed: Option<Expr>,

    pub raw_extras_chat_message: Option<Expr>,
    pub extras_chat_message: Option<Expr>,
}

/// Helper to unwrap an optional callback.
fn unwrap(option: Option<CallbackInfo>) -> (TokenStream, Option<TokenStream>) {
    if let Some(callback) = option {
        (callback.function, Some(callback.value))
    } else {
        (quote! {}, None)
    }
}

impl ExtrasGen {
    /// Generates Unofficial Extras exports.
    pub fn build(&self, name: LitStr) -> TokenStream {
        let (squad_update_func, squad_update_value) = unwrap(self.build_extras_squad_update());
        let (language_changed_func, language_changed_value) =
            unwrap(self.build_extras_language_changed());
        let (keybind_changed_func, keybind_changed_value) =
            unwrap(self.build_extras_keybind_changed());
        let (chat_message_func, chat_message_value) = unwrap(self.build_extras_chat_message());
        let init_func = self.build_extras_init(
            name,
            squad_update_value,
            language_changed_value,
            keybind_changed_value,
            chat_message_value,
        );

        quote! {
            #init_func
            #squad_update_func
            #language_changed_func
            #keybind_changed_func
            #chat_message_func
        }
    }

    /// Generates the extras init function.
    fn build_extras_init(
        &self,
        name: LitStr,
        squad_update: Option<TokenStream>,
        language_changed: Option<TokenStream>,
        keybind_changed: Option<TokenStream>,
        chat_message: Option<TokenStream>,
    ) -> TokenStream {
        let has_callback =
            squad_update.is_some() || language_changed.is_some() || keybind_changed.is_some();
        let squad_callback = squad_update.unwrap_or(quote! { ::std::option::Option::None });
        let lang_callback = language_changed.unwrap_or(quote! { ::std::option::Option::None });
        let keybind_callback = keybind_changed.unwrap_or(quote! { ::std::option::Option::None });
        let chat_callback = chat_message.unwrap_or(quote! { ::std::option::Option::None });

        let convert_addon = quote! {
            let addon = addon.as_ref().expect("unofficial extras did not provide addon info in init");
        };
        let subscribe = quote! {
            sub.as_mut()
                .expect("unofficial extras did not provide subscriber info in init")
                .subscribe(addon, #name, #squad_callback, #lang_callback, #keybind_callback, #chat_callback);
        };

        let content = if let Some(raw) = &self.raw_extras_init {
            let span = syn::Error::new_spanned(raw, "").span();
            quote_spanned! {span=>
                const RAW: ::arcdps::extras::callbacks::RawExtrasSubscriberInit = #raw;
                RAW(addon, sub)
            }
        } else if let Some(safe) = &self.extras_init {
            let span = syn::Error::new_spanned(safe, "").span();
            quote_spanned! {span=>
                const SAFE: ::arcdps::extras::callbacks::ExtrasInitFunc = #safe;

                #convert_addon
                #subscribe

                let user = ::arcdps::__macro::str_from_cstr(addon.self_account_name as _)
                    .map(::arcdps::__macro::strip_account_prefix);
                SAFE(addon.clone().into(), user);
            }
        } else if has_callback {
            quote! {
                #convert_addon
                #subscribe
            }
        } else {
            // we dont need the export
            return quote! {};
        };

        quote_spanned! {content.span()=>
            #[no_mangle]
            unsafe extern #SYSTEM_ABI fn arcdps_unofficial_extras_subscriber_init(
                addon: *const ::arcdps::extras::RawExtrasAddonInfo,
                sub: *mut ::arcdps::extras::ExtrasSubscriberInfo
            ) {
                #content
            }
        }
    }

    /// Generates the extras squad update callback.
    fn build_extras_squad_update(&self) -> Option<CallbackInfo> {
        CallbackInfo::build_optional(
            self.raw_extras_squad_update.as_ref(),
            self.extras_squad_update.as_ref(),
            quote! { abstract_extras_squad_update },
            |safe, span| {
                quote_spanned! {span=>
                    unsafe extern #C_ABI fn abstract_extras_squad_update(
                        users: *const ::arcdps::extras::user::UserInfo,
                        count: ::std::primitive::u64
                    ) {
                        const SAFE: ::arcdps::extras::callbacks::ExtrasSquadUpdateCallback = #safe;

                        SAFE(::arcdps::extras::user::to_user_info_iter(users, count))
                    }
                }
            },
        )
    }

    /// Generates the extras language changed callback.
    fn build_extras_language_changed(&self) -> Option<CallbackInfo> {
        CallbackInfo::build_optional(
            self.raw_extras_language_changed.as_ref(),
            self.extras_language_changed.as_ref(),
            quote! { abstract_extras_language_changed },
            |safe, span| {
                quote_spanned! {span=>
                    unsafe extern #C_ABI fn abstract_extras_language_changed(language: ::arcdps::evtc::Language) {
                        const SAFE: ::arcdps::extras::callbacks::ExtrasLanguageChangedCallback = #safe;

                        SAFE(language)
                    }
                }
            },
        )
    }

    /// Generates the extras keybind changed callback.
    fn build_extras_keybind_changed(&self) -> Option<CallbackInfo> {
        CallbackInfo::build_optional(
            self.raw_extras_keybind_changed.as_ref(),
            self.extras_keybind_changed.as_ref(),
            quote! { abstract_extras_keybind_changed },
            |safe, span| {
                quote_spanned! {span=>
                    unsafe extern #C_ABI fn abstract_extras_keybind_changed(changed: ::arcdps::extras::keybinds::RawKeybindChange) {
                        const SAFE: ::arcdps::extras::callbacks::ExtrasKeybindChangedCallback = #safe;

                        SAFE(changed.into())
                    }
                }
            },
        )
    }

    /// Generates the extras chat message callback.
    fn build_extras_chat_message(&self) -> Option<CallbackInfo> {
        CallbackInfo::build_optional(
            self.raw_extras_chat_message.as_ref(),
            self.extras_chat_message.as_ref(),
            quote! { abstract_extras_chat_message },
            |safe, span| {
                quote_spanned! {span=>
                    unsafe extern #C_ABI fn abstract_extras_chat_message(info: *const ::arcdps::extras::message::RawChatMessageInfo) {
                        const SAFE: ::arcdps::extras::callbacks::ExtrasChatMessageCallback = #safe;

                        let info = info.as_ref()
                            .expect("unofficial extras did not provide message info in chat message callback")
                            .into();
                        SAFE(&info)
                    }
                }
            },
        )
    }
}
