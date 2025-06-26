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
use std::{io::BufRead, str::Chars, string::IntoChars};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::lexer::token::Token;
pub trait Scanner<I: BufRead,>: IntoIterator {
    fn new(input: I,) -> Self;
}
#[derive(Debug, Clone,)]
pub struct AnnasulScanner<I: BufRead,> {
    input:      I,
    into_chars: IntoChars,
    state:      AnnasulScannerState,
}
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Clone, Hash, Ord, PartialOrd, PartialEq, Eq,)]
enum AnnasulScannerState {
    #[default]
    Initial,
    Whitespace,
    Comment,
}
#[derive(Debug, Clone,)]
pub struct AnnasulScannerIter<I: BufRead,> {
    annasul_scanner: AnnasulScanner<I,>,
}
impl<I: BufRead,> Scanner<I,> for AnnasulScanner<I,> {
    fn new(input: I,) -> Self {
        Self {
            input,
            into_chars: String::new().into_chars(),
            state: Default::default(),
        }
    }
}
impl<I: BufRead,> IntoIterator for AnnasulScanner<I,> {
    type IntoIter = AnnasulScannerIter<I,>;
    type Item = std::io::Result<Token,>;

    fn into_iter(self,) -> Self::IntoIter { AnnasulScannerIter { annasul_scanner: self, } }
}
impl<I: BufRead,> Iterator for AnnasulScannerIter<I,> {
    type Item = std::io::Result<Token,>;

    fn next(&mut self,) -> Option<Self::Item,> {
        let scanner = &mut self.annasul_scanner;
        loop {
            let c = 'c: {
                if let Some(c,) = scanner.into_chars.next() {
                    break 'c c;
                }
                let mut buf = String::new();
                if match scanner.input.read_line(&mut buf,) {
                    Ok(num,) => num,
                    Err(e,) => return Some(Err(e,),),
                } == 0
                {
                    return None;
                }
                scanner.into_chars = buf.into_chars();
                if let Some(c,) = scanner.into_chars.next() {
                    break 'c c;
                } else {
                    return None;
                }
            };
            match (&scanner.state, c,) {
                (AnnasulScannerState::Initial, c,) if c.is_whitespace() => {
                    scanner.state = AnnasulScannerState::Whitespace;
                }
                (AnnasulScannerState::Initial, c,) => {
                    panic!("unknown char '{c}'")
                }
                (AnnasulScannerState::Whitespace, c,) if c.is_whitespace() => {}
                (AnnasulScannerState::Whitespace, c,) => {
                    panic!("unknown char '{c}'")
                }
                (AnnasulScannerState::Comment, c,) => {
                    panic!("unknown char '{c}'")
                }
            }
        }
    }
}
