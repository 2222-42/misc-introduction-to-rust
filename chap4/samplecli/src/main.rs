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
    #[clap(name = "NUMBER", default_value = "0")]
    num: i32,
}

fn main() {
    let opts = Opts::parse();

    match opts.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }

    println!("The num is: {}", opts.num);

    println!("Is verbosity specified?: {}", opts.verbose);
}
