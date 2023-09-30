//! Interfaces for extracting information from events.

use crate::Event;

/// Forcefully extracts information from a combat event.
pub trait Extract: Sized {
    /// Extracts [`Self`] from the combat event.
    ///
    /// # Safety
    /// This is safe when the given event is a valid event to extract [`Self`] from.
    unsafe fn extract(event: &Event) -> Self;
}

/// Attempts to extract information from a combat event.
pub trait TryExtract: Sized + Extract {
    /// Attempts to extract [`Self`] from the combat event.
    #[inline]
    fn try_extract(event: &Event) -> Option<Self> {
        Self::can_extract(event).then(|| unsafe { Self::extract(event) })
    }

    /// Checks whether [`Self`] can be extracted from the event.
    fn can_extract(event: &Event) -> bool;
}

/// Helper to transmute [`Event`] fields.
macro_rules! transmute_field {
    ($event: ident . $field: ident as $ty: tt) => {{
        const _: () = {
            let end = ::memoffset::offset_of!(Event, $field) + ::std::mem::size_of::<$ty>();
            assert!(
                end <= ::std::mem::size_of::<Event>(),
                "transmute field outside of event",
            );
        };

        let field_ptr = ::std::ptr::addr_of!($event.$field);
        (*field_ptr.cast::<$ty>()).clone()
    }};
}

pub(crate) use transmute_field;
