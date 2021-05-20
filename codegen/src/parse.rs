use proc_macro2::{Span, TokenStream};
use syn::{
    parse::ParseStream, punctuated::Punctuated, Error, Expr, FieldValue, Lit, LitStr, Member, Token,
};

pub(crate) struct ArcDpsGen {
    pub name:                LitStr,
    pub sig:                 Expr,
    pub init:                Option<Expr>,
    pub release:             Option<Expr>,
    pub raw_wnd_nofilter:    Option<Expr>,
    pub raw_imgui:           Option<Expr>,
    pub raw_options_end:     Option<Expr>,
    pub raw_combat:          Option<Expr>,
    pub raw_wnd_filter:      Option<Expr>,
    pub raw_options_windows: Option<Expr>,
    pub raw_combat_local:    Option<Expr>,
    pub wnd_nofilter:        Option<Expr>,
    pub combat:              Option<Expr>,
    pub imgui:               Option<Expr>,
    pub options_end:         Option<Expr>,
    pub combat_local:        Option<Expr>,
    pub wnd_filter:          Option<Expr>,
    pub options_windows:     Option<Expr>,
}

impl syn::parse::Parse for ArcDpsGen {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fields: Punctuated<FieldValue, Token![,]> = Punctuated::parse_terminated(input)?;

        let mut gen: ArcDpsGen = Self {
            name: LitStr::new("", Span::call_site()),
            sig:  Expr::Verbatim(TokenStream::new()),

            init:    None,
            release: None,

            combat:          None,
            combat_local:    None,
            imgui:           None,
            options_end:     None,
            options_windows: None,
            wnd_filter:      None,
            wnd_nofilter:    None,

            raw_combat:          None,
            raw_combat_local:    None,
            raw_imgui:           None,
            raw_options_end:     None,
            raw_options_windows: None,
            raw_wnd_filter:      None,
            raw_wnd_nofilter:    None,
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

                    "combat" => {
                        gen.combat = Some(field.expr);
                        if gen.raw_combat.is_some() {
                            return Err(Error::new_spanned(
                                name,
                                "combat and raw_combat are exclusive",
                            ));
                        }
                    }
                    "combat_local" => {
                        gen.combat_local = Some(field.expr);
                        if gen.raw_combat_local.is_some() {
                            return Err(Error::new_spanned(
                                name,
                                "combat_local and raw_combat_local are exclusive",
                            ));
                        }
                    }
                    "imgui" => {
                        gen.imgui = Some(field.expr);
                        if gen.raw_imgui.is_some() {
                            return Err(Error::new_spanned(
                                name,
                                "imgui and raw_imgui are exclusive",
                            ));
                        }
                    }
                    "options_end" => {
                        gen.options_end = Some(field.expr);
                        if gen.raw_options_end.is_some() {
                            return Err(Error::new_spanned(
                                name,
                                "options_end and raw_options_end are exclusive",
                            ));
                        }
                    }
                    "options_windows" => {
                        gen.options_windows = Some(field.expr);
                        if gen.raw_options_windows.is_some() {
                            return Err(Error::new_spanned(
                                name,
                                "options_windows and raw_options_windows are exclusive",
                            ));
                        }
                    }
                    "wnd_filter" => {
                        gen.wnd_filter = Some(field.expr);
                        if gen.raw_wnd_filter.is_some() {
                            return Err(Error::new_spanned(
                                name,
                                "wnd_filter and raw_wnd_filter are exclusive",
                            ));
                        }
                    }
                    "wnd_nofilter" => {
                        gen.wnd_nofilter = Some(field.expr);
                        if gen.raw_wnd_nofilter.is_some() {
                            return Err(Error::new_spanned(
                                name,
                                "wnd_nofilter and raw_wnd_nofilter are exclusive",
                            ));
                        }
                    }

                    "raw_combat" => {
                        gen.raw_combat = Some(field.expr);
                        if gen.combat.is_some() {
                            return Err(Error::new_spanned(
                                name,
                                "raw_combat and combat are exclusive",
                            ));
                        }
                    }
                    "raw_combat_local" => {
                        gen.raw_combat_local = Some(field.expr);
                        if gen.combat_local.is_some() {
                            return Err(Error::new_spanned(
                                name,
                                "raw_combat_local and combat_local are exclusive",
                            ));
                        }
                    }
                    "raw_imgui" => {
                        gen.raw_imgui = Some(field.expr);
                        if gen.imgui.is_some() {
                            return Err(Error::new_spanned(
                                name,
                                "raw_imgui and imgui are exclusive",
                            ));
                        }
                    }
                    "raw_options_end" => {
                        gen.raw_options_end = Some(field.expr);
                        if gen.options_end.is_some() {
                            return Err(Error::new_spanned(
                                name,
                                "raw_options_end and options_end are exclusive",
                            ));
                        }
                    }
                    "raw_options_windows" => {
                        gen.raw_options_windows = Some(field.expr);
                        if gen.options_windows.is_some() {
                            return Err(Error::new_spanned(
                                name,
                                "raw_options_windows and options_windows are exclusive",
                            ));
                        }
                    }
                    "raw_wnd_filter" => {
                        gen.raw_wnd_filter = Some(field.expr);
                        if gen.wnd_filter.is_some() {
                            return Err(Error::new_spanned(
                                name,
                                "raw_wnd_filter and wnd_filter are exclusive",
                            ));
                        }
                    }
                    "raw_wnd_nofilter" => {
                        gen.raw_wnd_nofilter = Some(field.expr);
                        if gen.wnd_nofilter.is_some() {
                            return Err(Error::new_spanned(
                                name,
                                "raw_wnd_nofilter and wnd_nofilter are exclusive",
                            ));
                        }
                    }
                    _ => {
                        return Err(Error::new_spanned(
                            name,
                            format!("no field named {} exists", name),
                        ))
                    }
                }
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
