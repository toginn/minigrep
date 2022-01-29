use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

/** Boxはトレイトオブジェクトで、Errorを実装し、型を具体的に実装しなくていい */
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ファイルの解析
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    // ファイルの表示
    println!("With text:\n{}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough argments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
