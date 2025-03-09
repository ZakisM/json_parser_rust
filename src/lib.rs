use std::{ffi::OsString, str::FromStr};

use bumpalo::Bump;
use parser::parser::Parser;
use wide::WideString;

mod wide;

// SAFETY: Not safe
#[unsafe(no_mangle)]
pub unsafe extern "stdcall" fn get_key(input_str_ptr: *const u16, input_str_len: i32) -> *mut u16 {
    let input = unsafe { WideString::from_raw_parts(input_str_ptr, input_str_len) };
    // thing.push(" - rustified!");

    let Ok(input) = input.into_inner().into_string() else {
        panic!("hmm");
    };

    let bump = Bump::new();
    let parser = Parser::new(&input);

    let res = match parser.parse(&bump) {
        Ok(res) => format!("{:?}", res.flattened()),
        Err(e) => format!("{e}"),
    };

    WideString::from(OsString::from_str(&res).expect("should never fail")).to_c_wide()
}
