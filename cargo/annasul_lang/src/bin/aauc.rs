use std::{fs::File, io::BufReader, path::PathBuf};

use annasul_lang::lexer::scanner::{AnnasulScanner, Scanner};
use clap::{Parser, ValueHint};
#[derive(Debug, Parser)]
struct Cli {
    #[clap(short, long, value_hint = ValueHint::AnyPath)]
    output: Option<PathBuf>,
    #[clap(value_hint = ValueHint::FilePath)]
    inputs: Vec<PathBuf>,
}
fn main() {
    let args = Cli::parse();
    for input in args.inputs {
        println!("-----{:?}-----", input);
        let tokens: Vec<_> =
            AnnasulScanner::new(BufReader::new(File::open(input).unwrap()))
                .into_iter()
                .collect();
        println!("{:?}", tokens);
    }
}
