extern crate winapi;
use std::ptr;
use winapi::um::winuser::{ MF_STRING, MF_POPUP, MF_BYCOMMAND, MF_SEPARATOR, AppendMenuA, CreateMenu, RemoveMenu };
use winapi::shared::windef::{ HMENU };
use utils::string_utils::WindowsStringUtils;

pub struct SubMenu {
	id: u32,
	name: String,
	flags: u32,
}

impl SubMenu {
	pub fn new(id: u32, name: &str) -> Self {
		// MF_STRING is default submenu flag
		SubMenu { id, name: name.to_string(), flags: MF_STRING }
	}
	pub fn new_seperator() -> Self {
		SubMenu { id: 0, name: String::new(), flags: MF_SEPARATOR}
	}
	pub fn set_flags(&mut self, flags: u32) -> &mut Self {
		self.flags = flags;
		self
	}
	pub fn append_to_menu(&self, &h_parent_menu: &HMENU) {
		unsafe { AppendMenuA(h_parent_menu, self.flags, self.id as usize, self.name.as_LPCSTR()) };
	}
}

pub struct Menu {
	submenus: Vec<SubMenu>,
	name: String,
	h_self: HMENU,
	h_parent_menu: HMENU,
}

impl Menu {
	pub fn new(name: &str) -> Self {
		Menu { 
			name: name.to_string(), 
			submenus: Vec::new(), 
			h_self: unsafe { CreateMenu() }, 
			h_parent_menu: ptr::null_mut(), 
		}
	}
	pub fn add_submenu(&mut self, submenu: SubMenu) -> &mut Self {
		self.submenus.push(submenu);
		self
	}
	pub fn add_parent(&mut self, h_parent_menu: &mut HMENU) -> &mut Self{
		self.h_parent_menu = *h_parent_menu;
		self
	}
	pub fn show(&mut self) {
		if self.h_parent_menu == ptr::null_mut() {
			return;
		}

		// create the main popup menu. uses self handle as unique menu ID
		unsafe { AppendMenuA(self.h_parent_menu, MF_STRING | MF_POPUP, self.h_self as usize, self.name.as_LPCSTR()) };

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
	pub fn remove_submenu(&mut self, target_menu: HMENU) -> &mut Self{
		unimplemented!();
		self
	}

}