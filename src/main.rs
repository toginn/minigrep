use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

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
    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

/** Boxはトレイトオブジェクトで、Errorを実装し、型を具体的に実装しなくていい */
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ファイルの解析
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    // ファイルの表示
    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough argments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
