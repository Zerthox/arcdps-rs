use cfg_if::cfg_if;

pub const C_ABI: &str = {
    cfg_if! {
        if #[cfg(feature = "unwind")] {
            "C-unwind"
        } else {
            "C"
        }
    }
};

pub const SYSTEM_ABI: &str = {
    cfg_if! {
        if #[cfg(feature = "unwind")] {
            "system-unwind"
        } else {
            "system"
        }
    }
};
