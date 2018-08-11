extern crate winapi;
extern crate widestring;

use winapi::um::libloaderapi::{GetModuleHandleW};
use winapi::um::psapi::{GetModuleInformation, MODULEINFO};
use self::widestring::WideCString;
use std::ptr;
use std::mem;

//https://wiki.alliedmods.net/Signature_Scanning
pub struct SigScanner {
	// address of module in memory
	base_addr: *const u32,
	initialized: bool,
}

impl SigScanner {
	pub fn new() -> Self {


		SigScanner { base_addr: &0, initialized: true }
	}
	pub fn find_sig(sig: &FunctionSig) -> u32 {
		0
	}

	fn get_dll_mem_info(&mut self) {
		// gets module to self
		let hmodule = unsafe { GetModuleHandleW(ptr::null()) };
		let mut module_info: MODULEINFO;
		// GetModuleInformation(hmodule, hmodule, &mut module_info, mem::size_of::<MODULEINFO>() as u32);
	}
}

pub struct FunctionSig {
	// signature to scan for
	sig_str: String,
	// mask to ignore certain bytes
	sig_mask: String,
	sig_len: u32,
	sig_addr: *const u32,
}

impl FunctionSig {
	pub fn new(sig_str: String) -> Self {
		FunctionSig { sig_str, sig_mask: "".to_string(), sig_len: 0, sig_addr: &0 }
	}
}