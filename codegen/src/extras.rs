use crate::CallbackInfo;
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
        let squad_callback = squad_update.unwrap_or(quote! { None });
        let lang_callback = language_changed.unwrap_or(quote! { None });
        let keybind_callback = keybind_changed.unwrap_or(quote! { None });
        let chat_callback = chat_message.unwrap_or(quote! { None });

        let subscribe = quote! {
            sub.subscribe(addon, #name, #squad_callback, #lang_callback, #keybind_callback, #chat_callback);
        };

        let content = if let Some(raw) = &self.raw_extras_init {
            let span = syn::Error::new_spanned(&raw, "").span();
            quote_spanned! {span=>
                let raw = (#raw) as RawExtrasSubscriberInit;
                raw(addon, sub)
            }
        } else if let Some(safe) = &self.extras_init {
            let span = syn::Error::new_spanned(&safe, "").span();
            quote_spanned! {span=>
                let safe = (#safe) as ExtrasInitFunc;

                #subscribe

                let user = ::arcdps::__macro::str_from_cstr(addon.self_account_name as _)
                    .map(::arcdps::__macro::strip_account_prefix);

                safe(addon.clone().into(), user);
            }
        } else if has_callback {
            quote! {
                #subscribe
            }
        } else {
            // we dont need the export
            return quote! {};
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

    /// Generates the extras squad update callback.
    fn build_extras_squad_update(&self) -> Option<CallbackInfo> {
        CallbackInfo::build_optional(
            self.raw_extras_squad_update.as_ref(),
            self.extras_squad_update.as_ref(),
            quote! { abstract_extras_squad_update },
            |safe, span| {
                quote_spanned! {span=>
                    unsafe extern "C" fn abstract_extras_squad_update(
                        users: *const ::arcdps::extras::RawUserInfo,
                        count: u64
                    ) {
                        let safe = (#safe) as ExtrasSquadUpdateCallback;
                        safe(::arcdps::extras::to_user_info_iter(users, count))
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
                    unsafe extern "C" fn abstract_extras_language_changed(language: ::arcdps::api::Language) {
                        let safe = (#safe) as ExtrasLanguageChangedCallback;
                        safe(language)
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
                    unsafe extern "C" fn abstract_extras_keybind_changed(changed: ::arcdps::extras::keybinds::RawKeybindChange) {
                        let safe = (#safe) as ExtrasKeybindChangedCallback;
                        safe(changed.into())
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
                    unsafe extern "C" fn abstract_extras_chat_message(info: *const ::arcdps::extras::message::RawChatMessageInfo) {
                        let safe = (#safe) as ExtrasChatMessageCallback;
                        safe(&::arcdps::extras::message::ChatMessageInfo::from(&*info))
                    }
                }
            },
        )
    }
}
