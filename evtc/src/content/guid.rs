use std::mem;
pub use windows_core::GUID;

/// Extensions for [`GUID`].
pub trait GuidExt {
    /// Formats the GUID as a simple hex string.
    fn format_simple(&self) -> String;

    /// Formats the GUID as a hyphenated hex string.
    fn format_hyphenated(&self) -> String;

    /// Returns the contained GUID **misinterpreted** as raw bytes.
    ///
    /// Some GW2 community projects misinterpret the memory layout of the GUID as bytes rather than a Windows [`GUID`].
    /// This is helpful when comparing or interfacing with such projects.
    ///
    /// # Safety
    /// The returned bytes represent the memory of the underlying Windows [`GUID`] struct.
    /// They do not represent the actual GUID.
    /// Constructing a GUID with them will result in a different GUID than the original.
    ///
    /// To get the correct bytes you can convert the GUID to a [`u128`] and then to bytes.
    unsafe fn misinterpret(&self) -> [u8; 16];
}

impl GuidExt for GUID {
    #[inline]
    fn format_simple(&self) -> String {
        format!("{:0>32X}", self.to_u128())
    }

    #[inline]
    fn format_hyphenated(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    unsafe fn misinterpret(&self) -> [u8; 16] {
        mem::transmute::<GUID, [u8; 16]>(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{content::ContentInfo, Event, StateChange, TryExtract};
    use std::mem;

    #[test]
    fn extract() {
        let event = Event {
            is_statechange: StateChange::IdToGUID.into(),
            src_agent: 4820869827943421467,
            dst_agent: 11091919494850445953,
            skill_id: 446,
            overstack_value: 0,
            ..unsafe { mem::zeroed() }
        };
        assert_eq!(event.src_agent, 0x42E72B9102F7561B);
        assert_eq!(event.dst_agent, 0x99EE6A0357CA8281);

        let info = ContentInfo::try_extract(&event).expect("failed to extract");
        assert_eq!(
            info.guid,
            GUID::from_u128(0x02F7561B_2B91_42E7_8182_CA57036AEE99)
        );
    }

    #[test]
    fn guid_format() {
        let guid = GUID::from_u128(0x02F7561B_2B91_42E7_8182_CA57036AEE99);

        assert_eq!(guid.format_simple(), "02F7561B2B9142E78182CA57036AEE99");
        assert_eq!(
            guid.format_hyphenated(),
            "02F7561B-2B91-42E7-8182-CA57036AEE99"
        );
    }
}
