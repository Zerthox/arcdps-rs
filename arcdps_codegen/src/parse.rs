use crate::ArcDpsGen;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Expr, FieldValue, Lit, Member, Token,
};

/// Helper to generate parsing.
macro_rules! match_parse {
    ($ident:expr, $gen:expr, $field:expr, $($name:ident),+; extras: { $($extras:ident),+ }) => {
        paste::paste! {
            match $ident.to_string().as_str() {
                $(
                    stringify!([<raw_ $name>]) => {
                        $gen.[<raw_ $name>] = Some($field.expr);
                        if $gen.$name.is_some() {
                            return Err(Error::new_spanned(
                                $ident,
                                stringify!([<raw_ $name>] and $name are exclusive),
                            ));
                        }
                    }
                    stringify!($name) => {
                        $gen.$name = Some($field.expr);
                        if $gen.[<raw_ $name>].is_some() {
                            return Err(Error::new_spanned(
                                $ident,
                                stringify!($name and [<raw_ $name>] are exclusive),
                            ));
                        }
                    }
                )+
                $(
                    #[cfg(feature = "extras")]
                    stringify!([<raw_ $extras>]) => {
                        $gen.extras.[<raw_ $extras>] = Some($field.expr);
                        if $gen.extras.$extras.is_some() {
                            return Err(Error::new_spanned(
                                $ident,
                                stringify!([<raw_ $extras>] and $extras are exclusive),
                            ));
                        }
                    }
                    #[cfg(feature = "extras")]
                    stringify!($extras) => {
                        $gen.extras.$extras = Some($field.expr);
                        if $gen.extras.[<raw_ $extras>].is_some() {
                            return Err(Error::new_spanned(
                                $ident,
                                stringify!($extras and [<raw_ $extras>] are exclusive),
                            ));
                        }
                    }
                    #[cfg(not(feature = "extras"))]
                    stringify!([<raw_ $extras>]) | stringify!($extras) => {
                        return Err(Error::new_spanned(
                            $ident,
                            format!("field {} requires the extras feature", $ident),
                        ));
                    }
                )+
                _ => return Err(Error::new_spanned(
                    $ident,
                    format!("no field named {} exists", $ident),
                )),
            }
        }
    }
}

impl Parse for ArcDpsGen {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fields: Punctuated<FieldValue, Token![,]> = Punctuated::parse_terminated(input)?;

        let mut gen = Self::default();
        let mut sig_done = false;

        for field in fields.into_iter() {
            if let Member::Named(ident) = &field.member {
                match ident.to_string().as_str() {
                    "name" => {
                        gen.name = if let Expr::Lit(expr) = field.expr {
                            if let Lit::Str(lit) = expr.lit {
                                Some(lit)
                            } else {
                                return Err(Error::new_spanned(
                                    expr,
                                    "name needs to be a literal of type &'static str",
                                ));
                            }
                        } else {
                            return Err(Error::new_spanned(
                                field.expr,
                                "name needs to be a literal of type &'static str",
                            ));
                        };
                    }
                    "sig" => {
                        sig_done = true;
                        gen.sig = field.expr;
                    }

                    "init" => gen.init = Some(field.expr),
                    "release" => gen.release = Some(field.expr),
                    "update_url" => gen.update_url = Some(field.expr),

                    _ => {
                        match_parse!(
                            ident,
                            gen,
                            field,
                            combat,
                            combat_local,
                            imgui,
                            options_end,
                            options_windows,
                            wnd_filter,
                            wnd_nofilter;
                            extras: {
                                extras_init,
                                extras_squad_update,
                                extras_language_changed,
                                extras_keybind_changed,
                                extras_chat_message
                            }
                        )
                    }
                };
            } else {
                return Err(Error::new_spanned(&field.member, "field must have a name"));
            }
        }

        if !sig_done {
            return Err(Error::new(input.span(), "sig field is required"));
        }

        Ok(gen)
    }
}
