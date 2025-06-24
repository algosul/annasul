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

use crate::lexer::token::Token;
use crate::lexer::token::Token::EOF;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Read};
use std::str::Chars;

pub trait Scanner<I: BufRead>: IntoIterator {
    fn new(input: I) -> Self;
}

#[derive(Debug, Clone)]
pub struct AnnasulScanner<'a, I: BufRead> {
    input: I,
    buf: String,
    chars: Chars<'a>,
    state: AnnasulScannerState,
}
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Clone, Hash, Ord, PartialOrd, PartialEq, Eq)]
enum AnnasulScannerState {
    #[default]
    Initial,
    Whitespace,
    Comment,
}
#[derive(Debug, Clone)]
pub struct AnnasulScannerIter<'a, I: BufRead> {
    annasul_scanner: AnnasulScanner<'a, I>,
}
impl<'a, I: BufRead> Scanner<I> for AnnasulScanner<'a, I> {
    fn new(input: I) -> Self {
        Self {
            input,
            buf: String::new(),
            chars: "".chars(),
            state: Default::default(),
        }
    }
}
impl<'a, I: BufRead> IntoIterator for AnnasulScanner<'a, I> {
    type Item = std::io::Result<Token>;
    type IntoIter = AnnasulScannerIter<'a, I>;

    fn into_iter(self) -> Self::IntoIter {
        AnnasulScannerIter {
            annasul_scanner: self,
        }
    }
}
impl<'a, I: BufRead> Iterator for AnnasulScannerIter<'a, I> {
    type Item = std::io::Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let scanner = &mut self.annasul_scanner;
        let c = if let Some(c) = scanner.chars.next() {
            c
        } else if match scanner.input.read_line(&mut scanner.buf) {
            Ok(num) => num,
            Err(e) => return Some(Err(e)),
        } != 0
            && let Some(c) = scanner.chars.next()
        {
            c
        } else {
            return None;
        };
        None
        // match (&scanner.state, scanner.buf.next()) {
        //     (AnnasulScannerState::Initial, c) if c.is_ascii_whitespace() => {}
        //     (AnnasulScannerState::Whitespace, _) => {}
        //     (AnnasulScannerState::Comment, _) => {}
        // }
    }
}
