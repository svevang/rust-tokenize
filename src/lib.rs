#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate natural;
use natural::tokenize::tokenize;

extern crate libc;
use std::ffi::{CString, CStr };
use std::os::raw::c_char;

fn into_safe_string(message: *const c_char) -> String {
    unsafe {
        CStr::from_ptr(message).to_string_lossy().into_owned()
    }
}


#[no_mangle]
pub extern fn hello_rust(raw: *const c_char) -> *const c_char {

    let incoming_message = into_safe_string(raw);
    let mut tokens = tokenize(&incoming_message);
    tokens.dedup();

    let response = CString::new(tokens.join(" ")).unwrap();
    let raw = response.into_raw();

    raw
}
