use colored::Colorize;
use std::process::exit;
use std::time::UNIX_EPOCH;

use crate::internal::AppExitCode;

const LOGO_SYMBOL: &str = "μ";
const ERR_SYMBOL: &str = "❌";

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {
        $crate::internal::strings::info_fn(format!($($arg)+));
    }
}

#[macro_export]
macro_rules! log {
    ($verbose:expr, $($arg:tt)+) => {
        $crate::internal::strings::log_fn(format!("{}:{} {}", file!(), line!(), format!($($arg)+)), $verbose);
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
    println!("{} {}", LOGO_SYMBOL.black(), msg.bold());
}

pub fn log_fn<S: ToString>(msg: S, verbose: bool) {
    let msg = msg.to_string();
    if verbose {
        eprintln!(
            "{} {}",
            std::time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            msg
        );
    }
}

pub fn crash_fn<S: ToString>(msg: S, exit_code: AppExitCode) {
    let msg = msg.to_string();
    println!("{} {}", ERR_SYMBOL.red(), msg.bold());
    exit(exit_code as i32);
}
