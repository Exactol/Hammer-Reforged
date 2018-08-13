extern crate winapi;
use std::ptr;
use std::mem;
use winapi::shared::minwindef::{ LPARAM, WPARAM, LRESULT };
use winapi::shared::windef::{ HWND };
use winapi::um::winuser::*;
use utils::resource::*;

//https://github.com/recombinant/PetzoldPW5e-rs

pub struct SubclassedWndProc {
	old_wnd_proc: WNDPROC,
	temp: i32,
}

impl SubclassedWndProc {
	pub fn new(hwnd: &HWND) -> Self {
		let mut new_proc = SubclassedWndProc{ old_wnd_proc: None, temp: 2 };

		// Subclasses window and gets the old WndProc location
		// let proc_location = unsafe { SetWindowLongA(*hwnd, GWL_WNDPROC, SubclassedWinProc::wnd_proc as i32) } as isize;
		let proc_location = unsafe { SetWindowLongA(*hwnd, GWL_WNDPROC, wnd_proc as i32) } as isize;

		// convert old wndproc to function pointer
		let old_wnd_proc = unsafe {
			mem::transmute::<isize, unsafe extern "system" fn(HWND, u32, usize, isize) -> LRESULT>(proc_location) 
		};
		new_proc.old_wnd_proc = Some(old_wnd_proc);
		println!("Old Wnd Proc: {}", proc_location);
		unsafe { g_old_wnd_proc = Some(old_wnd_proc) };
		new_proc
	}

	// TODO: find way to get funtion pointer to self

	// #[no_mangle]
	// pub unsafe extern "system" fn wnd_proc(&self, hwnd: HWND, message: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
	// 	println!("temp: {}", unsafe { test });
	// 	println!("old global wnd proc: {}", unsafe { g_old_wnd_proc.unwrap() } as i32);
	// 	println!("old wnd proc: {}", self.old_wnd_proc.unwrap() as i32);
	// 	match message {
	// 		WM_COMMAND => {
	// 			// handle button presses
	// 			match wParam as u32 {
	// 				ID_EXIT => {
	// 					println!("Exit pressed");
	// 				}
	// 				_ => {}
	// 			}
	// 		}
	// 		_ => {}
	// 	}
	// 	CallWindowProcA(self.old_wnd_proc, hwnd, message, wParam, lParam)
	// }
}

#[no_mangle] 
#[allow(private_no_mangle_fns)]
unsafe extern "system" fn wnd_proc(hwnd: HWND, message: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
	match message {
		WM_COMMAND => {
			// handle button presses
			match wParam as u32 {
				ID_EXIT => {
					println!("Exit pressed");
				}
				_ => {}
			}
		}
		// hacky workaround to make custom menu items appeared enabled. Breaks recent items
		// TODO: find better workaround
		WM_INITMENUPOPUP => {
			return 0;
		}
		_ => {}
	}
	CallWindowProcA(g_old_wnd_proc, hwnd, message, wParam, lParam)
}

static mut g_old_wnd_proc: WNDPROC = None;