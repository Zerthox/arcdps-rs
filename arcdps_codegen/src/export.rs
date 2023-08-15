use crate::{abi::SYSTEM_ABI, ArcDpsGen};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::env;
use syn::LitStr;

impl ArcDpsGen {
    /// Generates a literal with the plugin's name.
    pub fn gen_name(&self) -> LitStr {
        let (raw_name, span) = if let Some(input_name) = &self.name {
            let name = input_name.value();
            (name, input_name.span())
        } else {
            let name = env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME is not set");
            (name, Span::call_site())
        };
        LitStr::new(raw_name.as_str(), span)
    }

    /// Generates a null-terminated literal with the plugin's name.
    pub fn gen_name_cstr(&self) -> LitStr {
        let name = self.gen_name();

        LitStr::new((name.value() + "\0").as_str(), name.span())
    }

    /// Generates the build/version of the plugin as null-terminated literal.
    pub fn gen_build(&self) -> LitStr {
        let build = env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION is not set") + "\0";
        LitStr::new(build.as_str(), Span::call_site())
    }

    /// Generates the plugin's exports.
    pub fn build_export(&self) -> TokenStream {
        let name = self.gen_name();
        let name_c = self.gen_name_cstr();
        let build = self.gen_build();
        let sig = &self.sig;

        let init = self.build_init();
        let release = self.build_release();
        let update_url = self.build_update_url();

        let (combat_func, combat_value) = self.build_combat().into_tuple();
        let (combat_local_func, combat_local_value) = self.build_combat_local().into_tuple();
        let (imgui_func, imgui_value) = self.build_imgui().into_tuple();
        let (options_end_func, options_end_value) = self.build_options_end().into_tuple();
        let (options_windows_func, options_windows_value) =
            self.build_options_windows().into_tuple();
        let (wnd_filter_func, wnd_filter_value) = self.build_wnd_filter().into_tuple();
        let (wnd_nofilter_func, wnd_nofilter_value) = self.build_wnd_nofilter().into_tuple();

        // TODO: instantiate unused supported fields for intellisense & type checking?

        quote! {
            /// ArcDPS export struct with plugin information.
            static EXPORT: ::arcdps::callbacks::ArcDpsExport = ::arcdps::callbacks::ArcDpsExport {
                size: ::std::mem::size_of::<::arcdps::callbacks::ArcDpsExport>(),
                sig: #sig,
                imgui_version: 18000,
                out_build: #build.as_ptr() as _,
                out_name: #name_c.as_ptr() as _,
                combat: #combat_value,
                combat_local: #combat_local_value,
                imgui: #imgui_value,
                options_end: #options_end_value,
                options_windows: #options_windows_value,
                wnd_filter: #wnd_filter_value,
                wnd_nofilter: #wnd_nofilter_value,
            };

            /// ArcDPS export struct with error information.
            static mut EXPORT_ERROR: ::arcdps::callbacks::ArcDpsExport = ::arcdps::callbacks::ArcDpsExport {
                size: 0,
                sig: 0,
                imgui_version: 18000,
                out_build: #build.as_ptr() as _,
                out_name: #name_c.as_ptr() as _,
                combat: ::std::option::Option::None,
                combat_local: ::std::option::Option::None,
                imgui: ::std::option::Option::None,
                options_end: ::std::option::Option::None,
                options_windows: ::std::option::Option::None,
                wnd_filter: ::std::option::Option::None,
                wnd_nofilter: ::std::option::Option::None,
            };
            static mut ERROR_STRING: ::std::string::String = ::std::string::String::new();

            fn load() -> *const ::arcdps::callbacks::ArcDpsExport {
                let result: ::std::result::Result<(), ::std::string::String> = #init;
                if let ::std::result::Result::Err(err) = result {
                    unsafe {
                        ERROR_STRING = err + "\0";
                        EXPORT_ERROR.size = ERROR_STRING.as_ptr() as _;
                        &EXPORT_ERROR
                    }
                } else {
                    &EXPORT
                }
            }

            fn unload() {
                #release
            }

            /// ArcDPS looks for this exported function and calls the address it returns on client load.
            /// If you need any of the ignored values, create an issue with your use case.
            #[no_mangle]
            pub unsafe extern #SYSTEM_ABI fn get_init_addr(
                arc_version: *const ::arcdps::__macro::c_char,
                imgui_ctx: *mut ::arcdps::imgui::sys::ImGuiContext,
                id3d: *mut ::arcdps::__macro::c_void,
                arc_dll: ::arcdps::__macro::HMODULE,
                malloc: ::std::option::Option<::arcdps::__macro::MallocFn>,
                free: ::std::option::Option<::arcdps::__macro::FreeFn>,
                d3d_version: ::std::primitive::u32,
            ) -> fn() -> *const ::arcdps::callbacks::ArcDpsExport {
                ::arcdps::__macro::init(arc_version, arc_dll, imgui_ctx, malloc, free, id3d, d3d_version, #name);
                load
            }

            /// ArcDPS looks for this exported function and calls the address it returns on client exit.
            #[no_mangle]
            pub extern #SYSTEM_ABI fn get_release_addr() -> *mut ::arcdps::__macro::c_void {
                unload as _
            }

            #update_url

            #combat_func
            #combat_local_func
            #imgui_func
            #options_end_func
            #options_windows_func
            #wnd_filter_func
            #wnd_nofilter_func
        }
    }
}
