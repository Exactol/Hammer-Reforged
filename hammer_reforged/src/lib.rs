mod reforged;
mod sigscan;
mod ui;
mod utils;

extern crate winapi;
use std::thread;
use winapi::shared::minwindef::{HINSTANCE, DWORD, LPVOID, BOOL, TRUE};
use reforged::HammerReforged;

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
			// TODO: find way to rename thread
			// let builder = thread::Builder::new().name("Hammer Reforged".into());
			// let handler = builder.spawn(move || {
			// 	reforged::initialize();
			// }).unwrap();

			// handler.join().unwrap();

			// move to new thread to prevent deadlocks or something
			thread::spawn(move || {
				let mut ham_reforged = HammerReforged::new();
				ham_reforged.initialize();
			});
		},
		_ => {}
	}

	return TRUE;
}