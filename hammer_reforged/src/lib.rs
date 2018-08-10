extern crate winapi;

use std::ptr;
use std::thread;
use std::ffi::CString;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::{UINT, HINSTANCE, DWORD, LPVOID, BOOL, TRUE};
use winapi::shared::windef::{HWND};
use winapi::um::winuser::{MB_OK, MB_ICONINFORMATION, MessageBoxA};
use winapi::um::winnt::LPCWSTR;
use winapi::um::consoleapi::AllocConsole;
use winapi::um::processthreadsapi::CreateThread;
use std::io;

const DLL_PROCESS_DETACH: DWORD = 0;
const DLL_PROCESS_ATTACH: DWORD = 1;
const DLL_THREAD_ATTACH: DWORD = 2;
const DLL_THREAD_DETACH: DWORD = 3;

// entry point
#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(dll_handle: HINSTANCE, reason: DWORD, reserved: LPVOID) -> BOOL {
	match reason {
		DLL_PROCESS_DETACH => {
			// TODO cleanup
		},
		DLL_PROCESS_ATTACH => {
			thread::spawn(move || {
				initialize();
			});
		},
		_ => {}
	}

	return TRUE;
}

fn initialize() {
	unsafe { AllocConsole() };
	println!("test");
	println!("test");
	println!("test");
	println!("test");
	println!("test");

	// let lp_text = CString::new("Hello, world!").unwrap();
	// let lp_caption = CString::new("MessageBox Example").unwrap();
	// unsafe { MessageBoxA(
	// 	std::ptr::null_mut(),
	// 	lp_text.as_ptr(),
	// 	lp_caption.as_ptr(),
	// 	MB_OK | MB_ICONINFORMATION
	// ) };

	// let mut temp = String::new();
	// io::stdin().read_line(&mut temp);
}