use core::slice;
use std::{
    ffi::OsString,
    ops::{Deref, DerefMut},
    os::windows::ffi::{OsStrExt, OsStringExt},
};

#[derive(Debug)]
pub struct WideString(OsString);

impl WideString {
    pub unsafe fn from_raw_parts(data: *const u16, len: i32) -> Self {
        let wide =
            unsafe { slice::from_raw_parts(data, len.try_into().expect("invalid len passed")) };

        Self(OsString::from_wide(wide))
    }

    pub fn to_c_wide(&self) -> *mut u16 {
        let wide = self
            .encode_wide()
            .chain(std::iter::once(0))
            .collect::<Vec<_>>();

        unsafe { SysAllocString(wide.as_ptr()) }
    }
}

impl Deref for WideString {
    type Target = OsString;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WideString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[link(name = "oleaut32")]
unsafe extern "system" {
    fn SysAllocString(psz: *const u16) -> *mut u16;
}
