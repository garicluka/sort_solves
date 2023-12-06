use sort_solves::run;
use std::process::ExitCode;

pub fn main() -> ExitCode {
    if let Err(exit_code) = run() {
        return exit_code;
    };
    ExitCode::from(0)
}
