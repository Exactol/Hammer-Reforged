extern crate winapi;
extern crate widestring;
use std::mem;
use std::path;
use std::ptr;
use std::io::Error;
use std::ffi::CString;
use std::ffi::OsString;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStringExt;
use std::os::windows::ffi::OsStrExt;
use self::widestring::WideCString;
use self::winapi::um::synchapi::WaitForSingleObject;
use self::winapi::um::tlhelp32::{
	 PROCESSENTRY32W, CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS, Process32FirstW, Process32NextW
	 };
use self::winapi::um::handleapi::{ INVALID_HANDLE_VALUE, CloseHandle };
use self::winapi::um::processthreadsapi::{ OpenProcess, OpenProcessToken, GetCurrentProcess, CreateRemoteThread };
use self::winapi::um::winnt::{ 
	TOKEN_PRIVILEGES, TOKEN_ADJUST_PRIVILEGES, 
	TOKEN_QUERY, HANDLE, SE_PRIVILEGE_ENABLED, SE_DEBUG_NAME, LUID_AND_ATTRIBUTES, LUID,
	PROCESS_CREATE_THREAD, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
	MEM_RESERVE, MEM_COMMIT, PAGE_EXECUTE_READWRITE, LPCSTR, LPCWSTR
	};
use self::winapi::um::securitybaseapi::{ AdjustTokenPrivileges, };
use self::winapi::um::minwinbase::LPTHREAD_START_ROUTINE;
use self::winapi::um::winbase::{ LookupPrivilegeValueW, INFINITE };
use self::winapi::shared::minwindef::{ MAX_PATH, LPVOID, LPCVOID, FARPROC, HMODULE, TRUE, FALSE };
use self::winapi::shared::ntdef::NULL;
use self::winapi::um::memoryapi::{ VirtualAllocEx, WriteProcessMemory };
use self::winapi::um::libloaderapi::{ 
	GetProcAddress, GetModuleHandleW, GetModuleHandleExA, GetModuleHandleA, GetModuleFileNameW, 
	GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
	};


pub struct Injector<'a> {
	pub target_proc_name: &'a str,
	pub target_dll_path: &'a str,
	target_proc_id: u32,
	target_proc_handle: HANDLE,
}

impl<'a> Injector<'a> {
	pub fn new(target_proc_name: &'a str, target_dll_path: &'a str) -> Self {
		Injector {
			target_proc_name,
			target_dll_path,
			target_proc_id: 0,
			target_proc_handle: NULL,
		}
	}

	pub fn inject(&mut self) -> bool {
		println!("[+] Initializing Injector");
		
		// check that path exists
		if !path::Path::new(self.target_dll_path).exists() {
			println!("[!] Cannot find DLL path: {}", self.target_dll_path);
			return false;
		}

		if let Some(id) = self.find_proc_id() {
			// sucess
			self.target_proc_id = id;
		} else {
			// failed to find target proc id
			return false;
		}
		println!("[+] Process ID: {}", self.target_proc_id);
		println!("[+] DLL Filepath: {}", self.target_dll_path);

		match self.attach_to_process() {
			Some(proc_handle) => {
				self.target_proc_handle = proc_handle;
				println!("[+] Sucessfully attached");
			}
			None => {
				println!("[!] Failed to attach");
				return false;
			}
		};

		let lp_start_addr;
		let lp_exec_param;
		if let Some((start_addr, exec_param)) = self.alloc_write_dll() {
			// cast FARPROC to LPTHREAD_START_ROUTINE
			lp_start_addr = unsafe {mem::transmute::<FARPROC, LPTHREAD_START_ROUTINE>(start_addr) };
			lp_exec_param = exec_param;
		} else {
			println!("[!] Failed to allocate and write DLL");
			return false;
		}

		println!("[+] Creating remote thread");
		let thread = unsafe { CreateRemoteThread(self.target_proc_handle, ptr::null_mut(), 0, lp_start_addr, lp_exec_param, 0, ptr::null_mut()) };
		if thread == ptr::null_mut() {
			println!("[!] Remote thread execution failed");
			return false;
		}
		println!("[+] Remote thread created: {:p}", thread);
        unsafe { WaitForSingleObject(thread,INFINITE) };
		println!("[+] Finished");
		// TODO cleanup

		true
	}

	fn find_proc_id(&self) -> Option<u32> {
		println!("[+] Attempting to find target process ID");
		let mut proc_info: PROCESSENTRY32W = PROCESSENTRY32W {
			dwSize: mem::size_of::<PROCESSENTRY32W>() as u32,
			cntUsage: 0,
			th32ProcessID: 0,
			th32DefaultHeapID: 0,
			th32ModuleID: 0,
			cntThreads: 0,
			th32ParentProcessID: 0,
			pcPriClassBase: 0,
			dwFlags: 0,
			szExeFile: [0; MAX_PATH],
		};

		// create snapshot of all processes to find target process ID
		let proc_snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };

		if proc_snapshot == INVALID_HANDLE_VALUE {
			println!("[!] Could not create proccess proc_snapshot.");
			println!("Error: {:?}", Error::last_os_error() );
			return None;
		}
	
		// Make sure first proc exists
		if unsafe { Process32FirstW(proc_snapshot, &mut proc_info) } != 1 {
			println!("[!] Failed to read process snapshot");
			println!("Error: {:?}", Error::last_os_error() );
			unsafe {CloseHandle(proc_snapshot); }
			return None;
		}

		// convert first proc name from wide string
		let proc_name = OsString::from_wide(&proc_info.szExeFile)
			.into_string()
			.expect("[!] Failed to convert string.");

		// remove trailing null chars
		let proc_name = proc_name.trim_matches(char::from(0));

		// check if first proc is the target proc
		if self.target_proc_name == proc_name {
			unsafe { CloseHandle(proc_snapshot) };
			return Some(proc_info.th32ProcessID);
		}

		// iterate over all processes in snapshots
		while unsafe { Process32NextW(proc_snapshot, &mut proc_info) } != 0 {
			let proc_name = OsString::from_wide(&proc_info.szExeFile)
				.into_string()
				.expect("[!] Failed to convert string.");
			let proc_name = proc_name.trim_matches(char::from(0));

			if self.target_proc_name == proc_name {
				unsafe { CloseHandle(proc_snapshot) };
				return Some(proc_info.th32ProcessID);
			}
		}
		
		unsafe { CloseHandle(proc_snapshot) };
		println!("[!] Could not find target process.");
		println!("Error: {:?}", Error::last_os_error() );
		// TODO attempt to start process
		None
	}

	fn attach_to_process(&self) -> Option<HANDLE> {
		println!("[+] Attempting to attach to process");
		match self.set_debug_privileges() {
			true => println!("[+] Succesfully escalated privileges"),
			false => {
				println!("[!] Failed to escalate privileges");
				return None;
			}
		}
		println!("[+] Attaching to process {}", &self.target_proc_id);
		let proc_handle = unsafe { 
			OpenProcess(
				PROCESS_CREATE_THREAD | PROCESS_QUERY_INFORMATION | PROCESS_VM_OPERATION | PROCESS_VM_WRITE | PROCESS_VM_READ, 
				FALSE,
				self.target_proc_id)
			};
		
		if proc_handle == ptr::null_mut() {
			println!("[!] Target process is null");
			return None;
		}
		println!("Proc handle: {:p}", proc_handle);
		Some(proc_handle)
	}

	fn set_debug_privileges(&self) -> bool {
		println!("[+] Attempting to escalate privileges");
		let mut privileges: TOKEN_PRIVILEGES = TOKEN_PRIVILEGES {
			PrivilegeCount: 0,
			Privileges: [
				LUID_AND_ATTRIBUTES {
					Luid: LUID { LowPart: 0, HighPart: 0 },
					Attributes: 0,
			}; 1],
		};
		let mut token: HANDLE = NULL;
		// attempt to escalate process privilages to debug
		if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut token)} != FALSE {
			let mut luid: LUID = LUID { LowPart: 0, HighPart: 0 };
			let debug_name_wide = WideCString::from_str("SeDebugPrivilege").unwrap();
			if unsafe { LookupPrivilegeValueW(ptr::null(), debug_name_wide.as_ptr(), &mut luid) } != FALSE {
				privileges.PrivilegeCount = 1;
				privileges.Privileges[0].Luid = luid;		
				privileges.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;
				if unsafe { AdjustTokenPrivileges(token, FALSE, &mut privileges, mem::size_of::<TOKEN_PRIVILEGES>() as u32, ptr::null_mut(), ptr::null_mut()) } == 0 {
					println!("[!] Failed to adjust token privileges to debug");
					println!("Error: {:?}", Error::last_os_error() );
					return false;
				}
			}

			let err = Error::last_os_error();
			if err.raw_os_error().unwrap() != 0 {
				println!("[!] Error: {:?}", Error::last_os_error() );
			}

			unsafe { CloseHandle(token); }
		} else {
			println!("[!] Failed to adjust token privileges to debug");
			println!("Error: {:?}", Error::last_os_error() );
			return false;			
		}

		true
	}

	// returns (LoadLibraryW address , DLL path address)
	fn alloc_write_dll(&self) -> Option<(FARPROC, LPVOID)> {
		println!("[+] Allocating space for DLL path");
		let path_os = OsString::from(self.target_dll_path);
		let path_str = path_os.as_os_str();
		let path_len = ((path_str.len() + 1) * mem::size_of::<u16>());

		// allocate space in target processes's memory for the dll path
		let dll_path_addr = unsafe { VirtualAllocEx(self.target_proc_handle, ptr::null_mut(),
			path_len, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE) };

		// write dll path to memory
		println!("[+] Writing into process space at {:p}", dll_path_addr);
		if unsafe { WriteProcessMemory(self.target_proc_handle, dll_path_addr,	
				path_str.encode_wide().collect::<Vec<_>>().as_ptr() as *const winapi::ctypes::c_void, path_len, ptr::null_mut()) } == 0 {
			println!("[!] Failed to write to process memory");
			println!("Error: {:?}", Error::last_os_error() );
			return None;
		}

		// find LoadLibraryW
		println!("[+] Looking for LoadLibraryW in kernel32");

		// get handle to kernal module
		let kernel_str = WideCString::from_str("Kernel32.dll").unwrap();
		let kernal_handle = unsafe { GetModuleHandleW(kernel_str.as_ptr()) };
		println!("[+] Found Kernel at: {:p}", kernal_handle);
		
		if kernal_handle == ptr::null_mut() {
			println!("[!] Could not find Kernel32.dll");
			return None
		}

		//get address of LoadLibraryW
		let libary_str = CString::new("LoadLibraryW").unwrap();
		let load_library_addr: FARPROC = unsafe { 
			GetProcAddress(
				kernal_handle, 
				libary_str.as_ptr())
		};
		if load_library_addr == ptr::null_mut() {
			println!("[!] Could not find LoadLibraryW in kernel32");
			return None;
		}

		println!("[+] Found LoadLibraryW at {:p}", load_library_addr);
		Some((load_library_addr, dll_path_addr))
	}
}

// source: https://www.reddit.com/r/rust/comments/37k1ij/converting_str_to_utf16/

pub trait win_str_conversion {
	fn to_LPCSTR(&self) -> LPCSTR;
	fn to_LPCVOID(&self) -> LPCVOID;
	fn to_LPCWSTR(&self) -> LPCWSTR;
	fn to_wide(&self) -> WideCString;
	fn to_cstr(&self) -> CString;
}

impl win_str_conversion for str {
	fn to_LPCSTR(&self) -> LPCSTR {
		// CString::new(self).unwrap().as_ptr()
		(self.as_ptr() as *const i8)
	}
	fn to_LPCVOID(&self) -> LPCVOID {
		CString::new(self).unwrap().as_ptr() as *const winapi::ctypes::c_void
	}
	fn to_LPCWSTR(&self) -> LPCWSTR {
		WideCString::from_str(self).unwrap().as_ptr()
	}
	fn to_wide(&self) -> WideCString {
		WideCString::from_str(self).unwrap()
	}
	fn to_cstr(&self) -> CString {
		CString::new(self).unwrap()
	}
}
// look into
//https://users.rust-lang.org/t/tidy-pattern-to-work-with-lpstr-mutable-char-array/2976/2