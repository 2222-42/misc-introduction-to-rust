use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
};

use clap::Parser;
#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "2222-42",
    about = "Super sawsome sample RPN calculattor"
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn run<R: BufRead>(reader: R, verbose: bool) {
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        // TODO: error handling
        let f = File::open(path).unwrap(); // ファイルハンドルの取得
        let reader = BufReader::new(f); // Bufreadは高水準で、システムコールの数を減らす
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}
