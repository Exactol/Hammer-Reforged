extern crate winapi;
use std::ffi::CString;
use std::mem;
use winapi::um::winnt::{ LPCSTR };

pub trait WindowsStringUtils {
	fn as_LPCSTR(&self) -> LPCSTR;
}

impl WindowsStringUtils for str {
	fn as_LPCSTR(&self) -> LPCSTR {
		let string = CString::new(self).unwrap();
		let p_string = string.as_ptr();
		// don't deallocate string when function ends so it can be used
		mem::forget(string); // TODO might be a source of a memory leak?
		p_string
	}
}