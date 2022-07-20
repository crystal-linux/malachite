use crate::internal::AppExitCode;
use colored::*;
use std::process::exit;

const LOGO_SYMBOL: &str = "μ";
const ERR_SYMBOL: &str = "❌";

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {
        $crate::internal::strings::info_fn(format!($($arg)+))
    }
}

#[macro_export]
macro_rules! crash {
    ($exit_code:expr, $($arg:tt)+) => {
        $crate::internal::strings::crash_fn(format!($($arg)+), $exit_code)
    }
}

pub fn info_fn<S: ToString>(msg: S) {
    let msg = msg.to_string();
    println!("{} {}", LOGO_SYMBOL.black(), msg.bold())
}

pub fn crash_fn<S: ToString>(msg: S, exit_code: AppExitCode) {
    let msg = msg.to_string();
    println!("{} {}", ERR_SYMBOL.red(), msg.bold());
    exit(exit_code as i32);
}
