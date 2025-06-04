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

use abuild::command;
// use clap::CommandFactory;

fn main() {
    // let matches = command::Cli::command().get_matches();
    // if matches.contains_id("generate-bash-completions") {
    //     let default = Cow::Borrowed(env!("CARGO_PKG_NAME"));
    //     let bin_name = std::env::current_exe().map_or(default.clone(), |p| {
    //         p.file_name()
    //             .map_or(default.clone(), |n| Cow::Owned(n.display().to_string()))
    //     });
    //     command::generate_completion(Bash, &bin_name, &mut std::io::stdout());
    //     return;
    // }
    let args = command::parse_args();
    println!("input: {:?}", args);
}
