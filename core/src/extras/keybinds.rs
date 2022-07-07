// TODO: add enums

#[derive(Debug)]
#[repr(C)]
pub struct KeybindChange {
    key_control: i32,
    key_index: i32,
    single_key: Key,
}

#[derive(Debug)]
#[repr(C)]
pub struct Key {
    pub device_type: i32,
    pub code: i32,
    pub modifier: i32,
}
