use crate::CombatEvent;

/// Forcefully extracts information from a combat event.
pub trait Extract: Sized {
    /// Extracts [`Self`] from the combat event.
    ///
    /// # Safety
    /// This is safe when the given event is a valid event to extract [`Self`] from.
    unsafe fn extract(event: &CombatEvent) -> Self;
}

/// Attempts to extract information from a combat event.
pub trait TryExtract: Sized + Extract {
    /// Attempts to extract [`Self`] from the combat event.
    #[inline]
    fn try_extract(event: &CombatEvent) -> Option<Self> {
        Self::can_extract(event).then(|| unsafe { Self::extract(event) })
    }

    /// Checks whether [`Self`] can be extracted from the event.
    fn can_extract(event: &CombatEvent) -> bool;
}
