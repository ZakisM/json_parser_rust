use wide::WideString;

mod wide;

// SAFETY: Not safe
#[no_mangle]
pub unsafe extern "stdcall" fn get_key(input_str_ptr: *const u16, input_str_len: i32) -> *mut u16 {
    let mut thing = WideString::from_raw_parts(input_str_ptr, input_str_len);
    thing.push(" - rustified!");

    thing.to_c_wide()
}
