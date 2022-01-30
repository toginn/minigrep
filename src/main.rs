extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // unwrap_or_elseはエラー時にクロージャを呼び出す
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // 引数の表示
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // Okの戻り値がないときはunwrapを使わない
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
