use crate::types;


use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use winapi::um::libloaderapi::GetModuleHandleA;

use std::arch::asm;

pub fn send_message(target: &str, message: &str) {
    let handle = unsafe { GetModuleHandleA(b"WeChatWin.dll\0".as_ptr() as _) };
    println!("WechatWin handle: {:p}", handle);

    let callAddr = unsafe { (handle as *const u8).add(0xCE6C80) };

    let uTarget = OsStr::new(target)
        .encode_wide()
        .chain(Some(0)) // add NULL termination
        .collect::<Vec<_>>();
    let wTarget = types::WxString {
        text: uTarget.as_ptr(),
        size: uTarget.len(),
        capacity: uTarget.len(),
        fill: [0; 8],
    };

    let uMessage = OsStr::new(message)
        .encode_wide()
        .chain(Some(0)) // add NULL termination
        .collect::<Vec<_>>();
    let wMessage = types::WxString {
        text: uMessage.as_ptr(),
        size: uMessage.len(),
        capacity: uMessage.len(),
        fill: [0; 8],
    };

    let buffer: [u8; 0x3B0] = [0; 0x3B0];
    let at_addr = [buffer.as_ptr(); 3];

    println!(
        "at_addr: {:p}, buffer: {:p}, wMessage: {:p}, wTarget: {:p}, callAddr: {:p}",
        &at_addr, &buffer, &wMessage, &wTarget, callAddr
    );

    unsafe {
        asm!(
            "push {buffer}",
            "push 0",
            "push 1",
            "push {at_addr}",
            "push {wMessage}",
            "mov edx,{wTarget}",
            "mov ecx,{buffer}",
            "mov ebx,{callAddr}",
            "call ebx",
            at_addr = in(reg) at_addr.as_ptr(),
            buffer = in(reg) buffer.as_ptr(),
            wMessage = in(reg) &wMessage,
            wTarget = in(reg) &wTarget,
            callAddr = in(reg) callAddr,
        );
    }
}
