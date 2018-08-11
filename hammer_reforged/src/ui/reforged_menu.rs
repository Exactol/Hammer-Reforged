extern crate winapi;
use std::ptr;
use winapi::um::winuser::{ MF_STRING, MF_POPUP, MF_BYCOMMAND, AppendMenuA, CreateMenu, RemoveMenu };
use winapi::shared::windef::{ HMENU };
use utils::string_utils::WindowsStringUtils;

pub struct ReforgedSubMenu {
	id: u32,
	name: String,
	flags: u32,
}

impl ReforgedSubMenu {
	pub fn new(id: u32, name: &str) -> Self {
		// MF_STRING is default submenu flag
		ReforgedSubMenu { id, name: name.to_string(), flags: MF_STRING }
	}
	pub fn set_flags(&mut self, flags: u32) {
		self.flags = flags;
	}

	pub fn append_to_menu(&self, &h_parent_menu: &HMENU) {
		unsafe { AppendMenuA(h_parent_menu, self.flags, self.id as usize, self.name.as_LPCSTR()) };
	}
}

pub struct ReforgedMenu {
	submenus: Vec<ReforgedSubMenu>,
	name: String,
	h_self: HMENU,
	h_parent_menu: HMENU,
}

impl ReforgedMenu {
	pub fn new(name: &str) -> Self {
		ReforgedMenu { 
			name: name.to_string(), 
			submenus: Vec::new(), 
			h_self: ptr::null_mut(), 
			h_parent_menu: ptr::null_mut(), 
		}
	}
	pub fn add_submenu(&mut self, submenu: ReforgedSubMenu) {
		self.submenus.push(submenu);
	}
	pub fn show(&mut self, h_parent_menu: &mut HMENU) {
		self.h_self = unsafe { CreateMenu() };
		self.h_parent_menu = *h_parent_menu;

		// create the main popup menu. uses self handle as unique menu ID
		unsafe { AppendMenuA(*h_parent_menu, MF_STRING | MF_POPUP, self.h_self as usize, self.name.as_LPCSTR()) };

		// foreach submenu, append it to the main menu
		if !self.submenus.is_empty() {
			for submenu in self.submenus.iter() {
				submenu.append_to_menu(&self.h_self);
			}
		}
	}
	pub fn hide(&mut self) {
		// removes this menu from 
		unsafe { RemoveMenu(self.h_parent_menu, self.h_self as u32, MF_BYCOMMAND) };
	}
	pub fn remove_submenu(&mut self, target_menu: HMENU) {
		unimplemented!();
	}

}