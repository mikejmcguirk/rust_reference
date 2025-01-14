#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::correctness)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::restriction)]
#![warn(clippy::style)]
#![warn(clippy::suspicious)]
// Always allow
#![allow(clippy::allow_attributes_without_reason)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::mod_module_files)]
#![allow(clippy::needless_return)]
#![allow(clippy::print_stdout)]
#![allow(clippy::print_stderr)]

use std::{env, process::ExitCode};

pub mod howtos;

use crate::howtos::_001;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let Some(arg) = args.get(1) else {
        println!("No argument provided");
        return ExitCode::from(1);
    };

    // I know this is Yandev level programming
    #[expect(clippy::single_match_else)]
    match arg.as_str() {
        "001" => _001::start(),
        _ => {
            eprintln!("Unknown argument error");
            return ExitCode::from(1);
        }
    }

    return ExitCode::from(0);
}
