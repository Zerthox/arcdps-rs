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

        let subscribe = quote! {
            let addon = addon.as_ref().expect("unofficial extras did not provide addon info in init");
            sub.as_mut()
                .expect("unofficial extras did not provide subscriber info in init")
                .subscribe(addon, #name, #squad_callback, #lang_callback, #keybind_callback, #chat_callback);
        };

        let (globals, contents) = if let Some(raw) = &self.raw_extras_init {
            let span = syn::Error::new_spanned(raw, "").span();
            (
                quote_spanned! {span=>
                    const _EXTRAS_INIT: ::arcdps::extras::callbacks::RawExtrasSubscriberInit = #raw;

                },
                quote_spanned! {span=>
                    self::__EXTRAS_INIT(addon, sub)
                },
            )
        } else if let Some(safe) = &self.extras_init {
            let span = syn::Error::new_spanned(safe, "").span();
            (
                quote_spanned! {span=>
                    const __EXTRAS_INIT: ::arcdps::extras::callbacks::ExtrasInitFunc = #safe;
                },
                quote_spanned! {span=>
                    #subscribe

                    let user = ::arcdps::__macro::str_from_cstr(addon.self_account_name as _)
                        .map(::arcdps::__macro::strip_account_prefix);
                    self::__EXTRAS_INIT(addon.clone().into(), user);
                },
            )
        } else if has_callback {
            (quote! {}, subscribe)
        } else {
            // we dont need the export
            return quote! {};
        };

        quote_spanned! {contents.span()=>
            #globals

            #[no_mangle]
            unsafe extern #SYSTEM_ABI fn arcdps_unofficial_extras_subscriber_init(
                addon: *const ::arcdps::extras::RawExtrasAddonInfo,
                sub: *mut ::arcdps::extras::ExtrasSubscriberInfo
            ) {
                #contents
            }
        }
    }

    /// Generates the extras squad update callback.
    fn build_extras_squad_update(&self) -> Option<CallbackInfo> {
        CallbackInfo::build_optional(
            self.raw_extras_squad_update.as_ref(),
            self.extras_squad_update.as_ref(),
            "__extras_squad_update",
            |name, safe, span| {
                quote_spanned! {span=>
                    const __EXTRAS_SQUAD_UPDATE: ::arcdps::extras::callbacks::ExtrasSquadUpdateCallback = #safe;

                    unsafe extern #C_ABI fn #name(
                        users: *const ::arcdps::extras::user::UserInfo,
                        count: ::std::primitive::u64
                    ) {
                        self::__EXTRAS_SQUAD_UPDATE(::arcdps::extras::user::to_user_info_iter(users, count))
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
            "__extras_language_changed",
            |name, safe, span| {
                quote_spanned! {span=>
                    const __EXTRAS_LANGUAGE_CHANGED: ::arcdps::extras::callbacks::ExtrasLanguageChangedCallback = #safe;

                    unsafe extern #C_ABI fn #name(language: ::arcdps::evtc::Language) {
                        self::__EXTRAS_LANGUAGE_CHANGED(language)
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
            "__extras_keybind_changed",
            |name, safe, span| {
                quote_spanned! {span=>
                    const __EXTRAS_KEYBIND_CHANGED: ::arcdps::extras::callbacks::ExtrasKeybindChangedCallback = #safe;

                    unsafe extern #C_ABI fn #name(changed: ::arcdps::extras::keybinds::RawKeybindChange) {
                        self::__EXTRAS_KEYBIND_CHANGED(changed.into())
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
            "__extras_chat_message",
            |name, safe, span| {
                quote_spanned! {span=>
                    const __EXTRAS_CHAT_MESSAGE: ::arcdps::extras::callbacks::ExtrasChatMessageCallback = #safe;

                    unsafe extern #C_ABI fn #name(info: *const ::arcdps::extras::message::RawChatMessageInfo) {
                        let info = info.as_ref()
                            .expect("unofficial extras did not provide message info in chat message callback")
                            .into();
                        self::__EXTRAS_CHAT_MESSAGE(&info)
                    }
                }
            },
        )
    }
}
