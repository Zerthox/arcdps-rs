use std::ops::RangeInclusive;

/// Helper for version checks.
#[derive(Debug, Clone)]
pub struct ExtrasVersion {
    /// Version of the API.
    ///
    /// Gets incremented whenever a function signature or behavior changes in a breaking way.
    pub api_version: u32,

    /// Highest known version of the [`ExtrasSubscriberInfo`] struct.
    ///
    /// Also determines the size of the subscriber info buffer in the init call.
    /// The buffer is only guaranteed to have enough space for known [`ExtrasSubscriberInfo`] versions.
    pub max_info_version: u32,
}

impl ExtrasVersion {
    /// Supported Unofficial Extras API version.
    pub const API: u32 = 2;

    /// Range of supported [`ExtrasSubscriberInfo`] versions.
    pub const SUB_INFO_RANGE: RangeInclusive<u32> = 1..=2;

    /// Minimum supported [`ExtrasSubscriberInfo`] version.
    pub const MIN_SUB_INFO: u32 = *Self::SUB_INFO_RANGE.start();

    /// Maximum supported [`ExtrasSubscriberInfo`] version.
    pub const MAX_SUB_INFO: u32 = *Self::SUB_INFO_RANGE.end();

    /// Minimum [`ExtrasSubscriberInfo`] version for message callback.
    pub const MESSAGE_CALLBACK: u32 = 2;

    /// Minimum [`ExtrasSubscriberInfo`] version for message callback 2.
    pub const MESSAGE_CALLBACK2: u32 = 3;

    /// Creates new version information.
    #[inline]
    pub const fn new(api_version: u32, max_info_version: u32) -> Self {
        Self {
            api_version,
            max_info_version,
        }
    }

    /// Checks compatibility with the Unofficial Extras addon.
    #[inline]
    pub const fn is_compatible(&self) -> bool {
        self.api_version == Self::API && self.max_info_version >= Self::MIN_SUB_INFO
    }

    /// Checks for compatibility and returns the highest supported [`ExtrasSubscriberInfo`] version supported by Unofficial Extras & the bindings.
    #[inline]
    pub fn get_version_to_use(&self) -> Option<u32> {
        self.is_compatible()
            .then(|| self.max_info_version.min(Self::MAX_SUB_INFO))
    }

    /// Whether the Unofficial Extras addon supports squad chat message callback.
    #[inline]
    pub const fn supports_squad_chat_message(&self) -> bool {
        self.max_info_version >= Self::MESSAGE_CALLBACK
    }

    /// Whether the Unofficial Extras addon supports chat message callback 2.
    #[inline]
    pub const fn supports_chat_message2(&self) -> bool {
        self.max_info_version >= Self::MESSAGE_CALLBACK2
    }
}
