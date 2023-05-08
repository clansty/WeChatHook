mod httpServer;
mod utils;
mod types;

use std::thread;

use win32console::console::WinConsole;
use winapi::shared::minwindef::HINSTANCE;
use windows::Win32::System::SystemServices::*;


#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => attach(),
        DLL_PROCESS_DETACH => (),
        _ => (),
    }

    true
}

fn attach() {
    WinConsole::alloc_console();
    WinConsole::set_title("Cat App!").unwrap();
    println!("nya, hello injection from Rust!");

    thread::spawn(move || {
        httpServer::start_server();
    });
}
