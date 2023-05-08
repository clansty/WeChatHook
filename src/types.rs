use rocket::serde::{Deserialize};
use std::os::raw::c_char;

#[repr(C)]
pub struct WxString {
    pub text: *const u16,
    pub size: usize,
    pub capacity: usize,
    pub fill: [c_char; 8],
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SendMessageParams {
    pub target: String,
    pub content: String,
}
