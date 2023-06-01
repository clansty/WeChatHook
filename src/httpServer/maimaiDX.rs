use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use crate::types;
use winapi::um::libloaderapi::GetModuleHandleA;

use std::arch::asm;
use std::thread;
use std::time::Duration;

// type GetArg1Function = extern "C" fn(i32) -> i32;

pub fn qrcode() {
    let handle = unsafe { GetModuleHandleA(b"WeChatWin.dll\0".as_ptr() as _) };
    println!("WechatWin handle: {:p}", handle);

    let callAddr = unsafe { (handle as *const u8).add(0xDA2450) };

    let getArg1: *const u8 = unsafe { (handle as *const u8).add(0x240EAB0) };

    let arg1: i32;

    unsafe {
        asm!(
            "push 576",
            "call {callAddr}",
            "mov {arg1}, eax",
            "add esp, 4",
            callAddr = in(reg) getArg1,
            arg1 = out(reg) arg1,
        );
    }

    let arg3 = OsStr::new("#bizmenu#<info><id>454854166</id><key>rselfmenu_1</key><status>end</status><content></content></info>")
        .encode_wide()
        .chain(Some(0)) // add NULL termination
        .collect::<Vec<_>>();
    let arg8 = OsStr::new("gh_6cfb73ca89e6")
        .encode_wide()
        .chain(Some(0)) // add NULL termination
        .collect::<Vec<_>>();
    let esi = types::WxString {
        text: arg3.as_ptr(),
        size: 0x65,
        capacity: 0x65,
        fill: [0; 8],
    };
    println!("CallAddr: {:p}, arg1: {:x}, arg3: {:p}, arg8: {:p}", callAddr, arg1, arg3.as_ptr(), arg8.as_ptr());

    unsafe {
        asm!(
            "mov ecx,{arg1}",
            "push 0",
            "push 0",
            "push 0xf",
            "push 0xf",
            "push {arg8}",
            "push 0",
            "push 0",
            "push 0x65",
            "push 0x65",
            "push {arg3}",
            "push 1",
            "call {callAddr}",
            arg1 = in(reg) arg1,
            arg3 = in(reg) arg3.as_ptr(),
            arg8 = in(reg) arg8.as_ptr(),
            callAddr = in(reg) callAddr,
        );
    }

    // thread::sleep(Duration::from_secs(30));

    // let result = unsafe { (*function)(arg1.as_ptr() as u8, 1, arg3.as_ptr() as u8, 0x65, 0x65, 0, 0, arg8.as_ptr() as u8, 0xf, 0xf, 0, 0) };
    // println!("result: {:p}", result);
}
