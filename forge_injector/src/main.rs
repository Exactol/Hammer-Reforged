pub mod lib;
use lib::Injector;
use std::env;

fn main() {
    let DLL_NAME = "HammerReforged.dll";
    // let DLL_NAME = "HammerSyncInjected.dll";
    let TARGET_NAME = "hammer.exe";

    // TODO change debug to release or find way to automate it
    let mut dll_path = env::current_exe().unwrap(); //.to_string_lossy() + "\\target\\debug\\" + DLL_NAME).to_string();
    dll_path.set_file_name(DLL_NAME);
    let dll_path = dll_path.to_str().unwrap();
    let mut injector = Injector::new(TARGET_NAME, &dll_path);
    match injector.inject() {
        false => std::process::exit(1),
        true => std::process::exit(0)
    }
}
