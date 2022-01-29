use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // 引数の解析
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    // 引数の表示
    println!("Searching for {}", query);
    println!("In file {}", filename);

    // ファイルの解析
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    // ファイルの表示
    println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
