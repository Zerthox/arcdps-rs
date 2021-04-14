mod parse;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::Expr;

// noinspection SpellCheckingInspection
#[proc_macro]
pub fn arcdps_export(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as parse::ArcDpsGen);
    let sig = input.sig;
    let build = std::env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION is not set") + "\0";
    let build = syn::LitStr::new(build.as_str(), Span::call_site());
    let name = input.name.value() + "\0";
    let name = syn::LitStr::new(name.as_str(), input.name.span());

    let (abstract_combat, cb_combat) = build_combat(input.raw_combat, input.combat);
    let (abstract_combat_local, cb_combat_local) =
        build_combat_local(input.raw_combat_local, input.combat_local);
    let (abstract_imgui, cb_imgui) = build_imgui(input.raw_imgui, input.imgui);
    let (abstract_options_end, cb_options_end) =
        build_options_end(input.raw_options_end, input.options_end);
    let (abstract_options_windows, cb_options_windows) =
        build_options_windows(input.raw_options_windows, input.options_windows);

    let cb_wnd_filter = match (input.raw_wnd_filter, input.wnd_filter) {
        (Some(raw), _) => {
            let span = syn::Error::new_spanned(&raw, "").span();
            quote_spanned! (span => Some(#raw as _) )
        }
        (_, Some(_safe)) => unimplemented!(),
        _ => quote! { None },
    };

    let cb_wnd_nofilter = match (input.raw_wnd_nofilter, input.wnd_nofilter) {
        (Some(raw), _) => {
            let span = syn::Error::new_spanned(&raw, "").span();
            quote_spanned! (span => Some(#raw as _) )
        }
        (_, Some(_safe)) => unimplemented!(),
        _ => quote! { None },
    };

    let export = quote! {
        ArcDpsExport {
            size: ::std::mem::size_of::<ArcDpsExport>(),
            sig: #sig,
            imgui_version: 18000,
            out_build: #build.as_ptr(),
            out_name: #name.as_ptr(),
            combat: #cb_combat,
            combat_local: #cb_combat_local,
            imgui: #cb_imgui,
            options_end: #cb_options_end,
            options_windows: #cb_options_windows,
            wnd_filter: #cb_wnd_filter,
            wnd_nofilter: #cb_wnd_nofilter,
        }
    };

    let init = if let Some(init) = input.init {
        let span = syn::Error::new_spanned(&init, "").span();
        quote_spanned! (span => (#init as InitFunc)();)
    } else {
        quote! {}
    };

    let release = if let Some(release) = input.release {
        let span = syn::Error::new_spanned(&release, "").span();
        quote_spanned! (span => (#release as ReleaseFunc)();)
    } else {
        quote! {}
    };

    let res = quote! {
        mod __arcdps_gen_export {
            use super::*;
            use ::std::os::raw::{c_char, c_void};
            use ::arcdps::imgui;
            use ::arcdps::helpers;
            use ::arcdps::ArcDpsExport;
            use ::arcdps::{InitFunc, ReleaseFunc};

            type LPARAM = isize;
            type LPVOID = *mut c_void;
            type UINT = u32;
            type WPARAM = usize;
            type PCCHAR = *mut c_char;
            type HWND = *mut c_void;
            type HANDLE = *mut c_void;

            #abstract_combat
            #abstract_combat_local
            #abstract_imgui
            #abstract_options_end
            #abstract_options_windows

            static EXPORT: ArcDpsExport = #export;

            fn load() -> &'static ArcDpsExport {
                #init
                &EXPORT
            }

            fn unload() {
                #release
            }

            #[no_mangle]
            // export -- arcdps looks for this exported function and calls the address it returns on client load
            // if you need any of the ignored values, create an issue with your use case
            pub unsafe extern "system" fn get_init_addr(
                _arcversion: PCCHAR,
                imguictx: *mut imgui::sys::ImGuiContext,
                _id3dd9: LPVOID,
                _arcdll: HANDLE,
                mallocfn: Option<unsafe extern "C" fn(sz: usize, user_data: *mut c_void) -> *mut c_void>,
                freefn: Option<unsafe extern "C" fn(ptr: *mut c_void, user_data: *mut c_void)>,
            ) -> fn() -> &'static ArcDpsExport {
                imgui::sys::igSetCurrentContext(imguictx);
                imgui::sys::igSetAllocatorFunctions(mallocfn, freefn, ::core::ptr::null_mut());
                CTX = Some(imgui::Context::current());
                UI = Some(imgui::Ui::from_ctx(CTX.as_ref().unwrap()));
                load
            }

            static mut CTX: Option<imgui::Context> = None;
            static mut UI: Option<imgui::Ui> = None;

            #[no_mangle]
            /* export -- arcdps looks for this exported function and calls the address it returns on client exit */
            pub extern "system" fn get_release_addr() -> LPVOID {
                unload as LPVOID
            }
        }
    };
    res.into()
}

fn build_options_windows(
    raw_options_windows: Option<Expr>,
    options_windows: Option<Expr>,
) -> (TokenStream, TokenStream) {
    let mut abstract_options_windows = quote! {};
    let cb_options_windows = match (raw_options_windows, options_windows) {
        (Some(raw), _) => {
            let span = syn::Error::new_spanned(&raw, "").span();
            quote_spanned!(span => Some(#raw as _) )
        }
        (_, Some(safe)) => {
            let span = syn::Error::new_spanned(&safe, "").span();
            abstract_options_windows = quote_spanned!(span =>
            unsafe fn abstract_options_windows(window_name: PCCHAR) -> bool {
                    #safe(helpers::get_str_from_pc_char(window_name))
            });
            quote_spanned!(span => Some(__arcdps_gen_export::abstract_options_windows as _) )
        }
        _ => quote! { None },
    };
    (abstract_options_windows, cb_options_windows)
}

fn build_options_end(
    raw_options_end: Option<Expr>,
    options_end: Option<Expr>,
) -> (TokenStream, TokenStream) {
    let mut abstract_options_end = quote! {};
    let cb_options_end = match (raw_options_end, options_end) {
        (Some(raw), _) => {
            let span = syn::Error::new_spanned(&raw, "").span();
            quote_spanned!(span => Some(#raw as _) )
        }
        (_, Some(safe)) => {
            let span = syn::Error::new_spanned(&safe, "").span();
            abstract_options_end = quote_spanned!(span =>
            unsafe fn abstract_options_end() {
                let ui = UI.as_ref().unwrap();
                #safe(ui)
            });
            quote_spanned!(span => Some(__arcdps_gen_export::abstract_options_end as _) )
        }
        _ => quote! { None },
    };
    (abstract_options_end, cb_options_end)
}

fn build_imgui(raw_imgui: Option<Expr>, imgui: Option<Expr>) -> (TokenStream, TokenStream) {
    let mut abstract_imgui = quote! {};
    let cb_imgui = match (raw_imgui, imgui) {
        (Some(raw), _) => {
            let span = syn::Error::new_spanned(&raw, "").span();
            quote_spanned!(span => Some(#raw as _) )
        }
        (_, Some(safe)) => {
            let span = syn::Error::new_spanned(&safe, "").span();
            abstract_imgui = quote_spanned!(span =>
            unsafe fn abstract_imgui(loading: u32) {
                let ui = UI.as_ref().unwrap();
                #safe(ui, loading != 0)
            });
            quote_spanned!(span => Some(__arcdps_gen_export::abstract_imgui as _) )
        }
        _ => quote! { None },
    };
    (abstract_imgui, cb_imgui)
}

fn build_combat_local(
    raw_combat_local: Option<Expr>,
    combat_local: Option<Expr>,
) -> (TokenStream, TokenStream) {
    let mut abstract_combat_local = quote! {};
    let cb_combat_local = match (raw_combat_local, combat_local) {
        (Some(raw), _) => {
            let span = syn::Error::new_spanned(&raw, "").span();
            quote_spanned!(span => Some(#raw as _) )
        }
        (_, Some(safe)) => {
            let span = syn::Error::new_spanned(&safe, "").span();
            abstract_combat_local = quote_spanned!(span =>
            unsafe fn abstract_combat_local(
                    ev: *mut CombatEvent,
                    src: *mut RawAgent,
                    dst: *mut RawAgent,
                    skill_name: PCCHAR,
                    id: u64,
                    revision: u64,
                ) {
                    let args = helpers::get_combat_args_from_raw(ev, src, dst, skill_name);
                    #safe(args.ev, args.src, args.dst, args.skill_name, id, revision)
            });
            quote_spanned!(span => Some(__arcdps_gen_export::abstract_combat_local as _) )
        }
        _ => quote! { None },
    };
    (abstract_combat_local, cb_combat_local)
}

fn build_combat(raw_combat: Option<Expr>, combat: Option<Expr>) -> (TokenStream, TokenStream) {
    let mut abstract_combat = quote! {};
    let cb_combat = match (raw_combat, combat) {
        (Some(raw), _) => {
            let span = syn::Error::new_spanned(&raw, "").span();
            quote_spanned!(span => Some(#raw as _) )
        }
        (_, Some(safe)) => {
            let span = syn::Error::new_spanned(&safe, "").span();
            abstract_combat = quote_spanned!(span =>
            unsafe fn abstract_combat(
                    ev: *mut CombatEvent,
                    src: *mut RawAgent,
                    dst: *mut RawAgent,
                    skill_name: PCCHAR,
                    id: u64,
                    revision: u64,
                ) {
                    let args = helpers::get_combat_args_from_raw(ev, src, dst, skill_name);
                    #safe(args.ev, args.src, args.dst, args.skill_name, id, revision)
            });
            quote_spanned!(span => Some(__arcdps_gen_export::abstract_combat as _) )
        }
        _ => quote! { None },
    };
    (abstract_combat, cb_combat)
}
