pub type ExportGetKey = unsafe extern "C" fn(control: i32, key_index: i32);

pub type ExportGetKeybind = unsafe extern "C" fn(control: i32);
