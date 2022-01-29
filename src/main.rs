extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // 引数の解析
    let args: Vec<String> = env::args().collect();

    // unwrap_or_elseはエラー時にクロージャを呼び出す
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // 引数の表示
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // Okの戻り値がないときはunwrapを使わない
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
