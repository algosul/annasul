// Copyright (c) 2025 air (https://yuanair.github.io).
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, version 3 of the License only.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
use std::{env::Args, fs::File, io::BufReader, path::PathBuf};

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
        let tokens: Vec<_> = AnnasulScanner::new(BufReader::new(File::open(input).unwrap()))
            .into_iter()
            .collect();
        println!("{:?}", tokens);
    }
}
