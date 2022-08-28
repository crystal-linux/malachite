use colored::Colorize;
use std::process::exit;
use std::time::UNIX_EPOCH;

use crate::internal::AppExitCode;

const LOGO_SYMBOL: &str = "μ";
const ERR_SYMBOL: &str = "❌";

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {
        $crate::internal::strings::info_fn(&format!($($arg)+));
    }
}

#[macro_export]
macro_rules! log {
    ($verbose:expr, $($arg:tt)+) => {
        $crate::internal::strings::log_fn(&format!("[{}:{}] {}", file!(), line!(), format!($($arg)+)), $verbose);
    }
}

#[macro_export]
macro_rules! crash {
    ($exit_code:expr, $($arg:tt)+) => {
        $crate::internal::strings::crash_fn(&format!("[{}:{}] {}", file!(), line!(), format!($($arg)+)), $exit_code)
    }
}

#[macro_export]
macro_rules! prompt {
    (default $default:expr, $($arg:tt)+) => {
        $crate::internal::strings::prompt_fn(&format!($($arg)+), $default)
    };
}

pub fn info_fn(msg: &str) {
    println!("{} {}", LOGO_SYMBOL.green(), msg.bold());
}

pub fn log_fn(msg: &str, verbose: bool) {
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

pub fn crash_fn(msg: &str, exit_code: AppExitCode) {
    println!("{} {}", ERR_SYMBOL.red(), msg.bold());
    exit(exit_code as i32);
}

pub fn prompt_fn(msg: &str, default: bool) -> bool {
    let yn = if default { "[Y/n]" } else { "[y/N]" };
    print!("{} {} {}", "?".bold().green(), msg.bold(), yn);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_lowercase();

    if input == "y" || input == "yes" {
        true
    } else if input == "n" || input == "no" {
        false
    } else {
        default
    }
}
