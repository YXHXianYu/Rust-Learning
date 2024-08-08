use std::env;
use std::process;

use minigrep::Config;

fn main() {

    let is_ignore_case = env::var("IS_IGNORE_CASE").is_ok();

    let config = Config::build(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Config building error: {}", err);
            process::exit(1);
        })
        .set_is_ignore_case(is_ignore_case);

    eprintln!("MiniGrep will search `{}` in File `{}`", config.target, config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
