extern crate winapi;

use winapi::shared::windef::{HWND, HMENU };
use winapi::shared::minwindef::{FALSE, TRUE};
use winapi::um::winuser::{MB_OK, MB_ICONINFORMATION, MessageBoxA, FindWindowA, GetMenu};
use winapi::um::winnt::{ LPCWSTR, MEMORY_BASIC_INFORMATION };
use winapi::um::consoleapi::AllocConsole;
use winapi::um::memoryapi::{VirtualQuery, };
use std::ptr;
use std::ffi::CString;
use std::io;
use std::mem;
use sigscan::SigScanner;
use utils::string_utils::WindowsStringUtils;
use utils::resource::*;
use ui::reforged_menu::*;

pub fn initialize() {
	unsafe { AllocConsole() };

	let mut config: Config;
	match Config::new() {
		Ok(conf) => config = conf,
		Err(err) => {
			println!("Config failed with error: {}", err);
			return;
		}
	}
	println!("Window handle: {:p}", config.h_hammer_wnd);
	initialize_menus(&mut config.h_hammer_menu);
	
}

// creates menus and their submenus
fn initialize_menus(h_hammer_menu: &mut HMENU) {
	let test_menu = SubMenu::new(ID_EXIT, "Hello Hammer");
	let test_menu2 = SubMenu::new(ID_EXIT, "Hello Hammer");
	let test_seperator = SubMenu::new_seperator();
	let mut test_parent = Menu::new("Test");

	test_parent.add_submenu(test_menu).add_submenu(test_seperator).add_submenu(test_menu2).add_parent(h_hammer_menu);
	test_parent.show();
}

struct Config {
	sig_scanner: SigScanner,
	h_hammer_wnd: HWND,
	h_hammer_menu: HMENU
}

impl Config {
	fn new() -> Result<Self, String> {
		// todo find better way to find window handle
		let h_hammer_wnd = unsafe { FindWindowA("VALVEWORLDCRAFT".as_LPCSTR(), ptr::null()) };

		if h_hammer_wnd == ptr::null_mut() {
			return Err("Failed to find hammer window".to_string());
		}

		let h_hammer_menu = unsafe { GetMenu(h_hammer_wnd) };

		if h_hammer_menu == ptr::null_mut() {
			return Err("Failed to find hammer menu".to_string());
		}		

		Ok(Config{ sig_scanner: SigScanner::new(), h_hammer_wnd, h_hammer_menu })
	}
}

