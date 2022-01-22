mod strings;
pub mod structs;

pub fn info(a: String) {
    strings::info(a);
}

pub fn crash(a: String, b: i32) {
    strings::crash(a, b);
}