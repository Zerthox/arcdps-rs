use crate::util::Share;
use std::{ffi::c_void, sync::OnceLock};
use windows::{
    Win32::Graphics::{Direct3D11::ID3D11Device, Dxgi::IDXGISwapChain},
    core::{Interface, InterfaceRef},
};

/// DirectX 11 swap chain.
static DXGI_SWAP_CHAIN: OnceLock<Share<InterfaceRef<'static, IDXGISwapChain>>> = OnceLock::new();

/// Returns the DirectX swap chain, if available.
#[inline]
pub fn dxgi_swap_chain() -> Option<IDXGISwapChain> {
    DXGI_SWAP_CHAIN
        .get()
        .map(|swap_chain| (*unsafe { swap_chain.get() }).to_owned())
}

/// Returns the DirectX 11 device, if available.
#[inline]
pub fn d3d11_device() -> Option<ID3D11Device> {
    let swap_chain = dxgi_swap_chain()?;
    unsafe { swap_chain.GetDevice() }.ok()
}

/// Initializes DirectX information.
pub unsafe fn init_dxgi(id3d: *mut c_void) {
    if !id3d.is_null() {
        let swap_chain =
            unsafe { IDXGISwapChain::from_raw_borrowed(&id3d) }.expect("invalid swap chain");
        DXGI_SWAP_CHAIN
            .get_or_init(|| unsafe { Share::new(InterfaceRef::from_interface(swap_chain)) });
    }
}
