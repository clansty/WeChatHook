use crate::utils;
use winapi::um::libloaderapi::GetModuleHandleA;

pub fn get_my_info() -> String {
    let handle = unsafe { GetModuleHandleA(b"WeChatWin.dll\0".as_ptr() as _) };
    println!("WechatWin handle: {:p}", handle);

    let wxid = utils::get_pointed_string(unsafe { (handle as *const u8).add(0x2FFD484) });
    let name = utils::get_addressed_string(unsafe { (handle as *const i8).add(0x2FFD590) });
    let phone = utils::get_addressed_string(unsafe { (handle as *const i8).add(0x2FFD500) });
    let avatar = utils::get_pointed_string(unsafe { (handle as *const u8).add(0x2FFD774) });
    println!(
        "wxid: {}, name: {}, phone: {}, avatar: {}",
        wxid, name, phone, avatar
    );

    let json = serde_json::json!({
        "wxid": wxid,
        "name": name,
        "phone": phone,
        "avatar": avatar,
    });
    return json.to_string();
}
