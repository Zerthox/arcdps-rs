use proc_macro2::{Span, TokenStream};
use syn::{
    parse::ParseStream, punctuated::Punctuated, Error, Expr, FieldValue, Lit, LitStr, Member, Token,
};

macro_rules! match_parse {
    ($test:expr, $gen:expr, $field:expr, $($name:expr),+) => {
        paste::paste! {
            match $test.to_string().as_str() {
                $(
                stringify!([<raw_ $name>]) => {
                    $gen.[<raw_ $name>] = Some($field.expr);
                    if $gen.$name.is_some() {
                        return Err(Error::new_spanned(
                            $test,
                            stringify!([<raw_ $name>] and $name are exclusive),
                        ));
                    }
                }
                stringify!($name) => {
                    $gen.$name = Some($field.expr);
                    if $gen.[<raw_ $name>].is_some() {
                        return Err(Error::new_spanned(
                            $test,
                            stringify!($name and [<raw_ $name>] are exclusive),
                        ));
                    }
                }
                ),+

                _ => {
                    return Err(Error::new_spanned(
                        $test,
                        format!("no field named {} exists", $test),
                    ))
                }
            }
        }
    }
}

pub(crate) struct ArcDpsGen {
    pub name: LitStr,
    pub sig: Expr,
    pub init: Option<Expr>,
    pub release: Option<Expr>,
    pub raw_wnd_nofilter: Option<Expr>,
    pub raw_imgui: Option<Expr>,
    pub raw_options_end: Option<Expr>,
    pub raw_combat: Option<Expr>,
    pub raw_wnd_filter: Option<Expr>,
    pub raw_options_windows: Option<Expr>,
    pub raw_combat_local: Option<Expr>,
    pub raw_unofficial_extras_init: Option<Expr>,
    pub raw_unofficial_extras_squad_update: Option<Expr>,
    pub wnd_nofilter: Option<Expr>,
    pub combat: Option<Expr>,
    pub imgui: Option<Expr>,
    pub options_end: Option<Expr>,
    pub combat_local: Option<Expr>,
    pub wnd_filter: Option<Expr>,
    pub options_windows: Option<Expr>,
    pub unofficial_extras_init: Option<Expr>,
    pub unofficial_extras_squad_update: Option<Expr>,
}

impl syn::parse::Parse for ArcDpsGen {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fields: Punctuated<FieldValue, Token![,]> = Punctuated::parse_terminated(input)?;

        let mut gen: ArcDpsGen = Self {
            name: LitStr::new("", Span::call_site()),
            sig:  Expr::Verbatim(TokenStream::new()),

            init:    None,
            release: None,

            combat:                         None,
            combat_local:                   None,
            imgui:                          None,
            options_end:                    None,
            options_windows:                None,
            wnd_filter:                     None,
            wnd_nofilter:                   None,
            unofficial_extras_init:         None,
            unofficial_extras_squad_update: None,

            raw_combat:                         None,
            raw_combat_local:                   None,
            raw_imgui:                          None,
            raw_options_end:                    None,
            raw_options_windows:                None,
            raw_wnd_filter:                     None,
            raw_wnd_nofilter:                   None,
            raw_unofficial_extras_init:         None,
            raw_unofficial_extras_squad_update: None,
        };

        let mut name_done = false;
        let mut sig_done = false;

        for field in fields.into_iter() {
            if let Member::Named(name) = &field.member {
                match name.to_string().as_str() {
                    "name" => {
                        name_done = true;
                        gen.name = if let Expr::Lit(expr) = field.expr {
                            if let Lit::Str(lit) = expr.lit {
                                lit
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

                    "init" => {
                        gen.init = Some(field.expr);
                    }

                    "release" => {
                        gen.release = Some(field.expr);
                    }

                    _ => {
                        match_parse!(
                            name,
                            gen,
                            field,
                            combat,
                            combat_local,
                            imgui,
                            options_end,
                            options_windows,
                            wnd_filter,
                            wnd_nofilter,
                            unofficial_extras_init,
                            unofficial_extras_squad_update
                        )
                    }
                };
            } else {
                return Err(Error::new_spanned(&field.member, "field must have a name"));
            }
        }

        if !name_done {
            return Err(Error::new(input.span(), "name field is required"));
        }
        if !sig_done {
            return Err(Error::new(input.span(), "sig field is required"));
        }

        Ok(gen)
    }
}
