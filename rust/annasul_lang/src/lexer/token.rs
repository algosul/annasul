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

/*!
 * a
 */

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Token {
    /// see [Comment]
    Comment(Comment),
    /// see [Identifier]
    Identifier(Identifier),
    /// see [Keyword]
    Keyword(Keyword),
    /// see [Literal]
    Literal(Literal),
    /// see [Operator]
    Operator(Operator),
    /// End of File
    EOF,
}

#[doc = include_str!("comment.md")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub struct Comment {
    /// see [CommentLineType]
    comment_line_type: CommentLineType,
    /// see [CommentType]
    comment_type: CommentType,
    comment: String,
}
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub enum CommentType {
    /// e.g. `/// comment`
    Inner,
    /// e.g. `//! comment`
    Outer,
    /// e.g. `// comment`
    Line,
}
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub enum CommentLineType {
    /// e.g. `// comment`
    SingleLine,
    /// e.g. `/* comment */`
    MultiLine,
}
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub struct Operator {
    inner: String,
}
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub struct Identifier {
    inner: String,
}
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Literal {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    #[cfg(feature = "unstable-f16")]
    F16(f16),
    F32(f32),
    F64(f64),
    #[cfg(feature = "unstable-f128")]
    F128(f128),
    Bool(bool),
    Char(char),
    String(String),
}
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub enum Keyword {
    r#abstract,
    r#as,
    r#async,
    r#await,
    r#become,
    r#box,
    r#break,
    r#const,
    r#continue,
    #[serde(rename = "crate")]
    crate_,
    r#do,
    r#dyn,
    r#else,
    r#enum,
    r#extern,
    r#false,
    r#final,
    r#fn,
    r#for,
    r#gen,
    r#if,
    r#impl,
    r#in,
    r#let,
    r#loop,
    r#marco,
    r#match,
    r#mod,
    r#move,
    r#mut,
    r#override,
    r#priv,
    r#pub,
    r#ref,
    r#return,
    #[serde(rename = "Self")]
    Self_,
    #[serde(rename = "self")]
    self_,
    r#static,
    r#struct,
    #[serde(rename = "super")]
    super_,
    r#trait,
    r#true,
    r#try,
    r#typeof,
    r#type,
    r#union,
    r#unsafe,
    r#unsized,
    r#use,
    r#virtual,
    r#where,
    r#while,
    r#yield,
    //
    r#u8,
    r#u16,
    r#u32,
    r#u64,
    r#u128,
    r#i8,
    r#i16,
    r#i32,
    r#i64,
    r#i128,
    #[cfg(feature = "unstable-f16")]
    r#f16,
    r#f32,
    r#f64,
    #[cfg(feature = "unstable-f128")]
    r#f128,
    r#bool,
    r#char,
    r#str,
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Comment(comm) => write!(f, "{}", comm),
            Token::Identifier(identifier) => write!(f, "{}", identifier),
            Token::Keyword(keyword) => write!(f, "{}", keyword),
            Token::Literal(literal) => write!(f, "{}", literal),
            Token::Operator(operator) => write!(f, "{}", operator),
            Token::EOF => write!(f, "<EOF>"),
        }
    }
}
impl Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match (&self.comment_line_type, &self.comment_type) {
            (CommentLineType::SingleLine, CommentType::Line) => {
                for line in self.comment.split('\n') {
                    writeln!(f, "//{}", line)?
                }
                Ok(())
            }
            (CommentLineType::MultiLine, CommentType::Line) => {
                write!(f, "/*{}*/", self.comment)
            }
            (CommentLineType::SingleLine, CommentType::Inner) => {
                for line in self.comment.split('\n') {
                    writeln!(f, "///{}", line)?
                }
                Ok(())
            }
            (CommentLineType::MultiLine, CommentType::Inner) => {
                write!(f, "/**{}*/", self.comment)
            }
            (CommentLineType::SingleLine, CommentType::Outer) => {
                for line in self.comment.split('\n') {
                    writeln!(f, "//!{}", line)?
                }
                Ok(())
            }
            (CommentLineType::MultiLine, CommentType::Outer) => {
                write!(f, "/*!{}*/", self.comment)
            }
        }
    }
}
impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::U8(val) => write!(f, "{:?}", val),
            Literal::U16(val) => write!(f, "{:?}", val),
            Literal::U32(val) => write!(f, "{:?}", val),
            Literal::U64(val) => write!(f, "{:?}", val),
            Literal::U128(val) => write!(f, "{:?}", val),
            Literal::I8(val) => write!(f, "{:?}", val),
            Literal::I16(val) => write!(f, "{:?}", val),
            Literal::I32(val) => write!(f, "{:?}", val),
            Literal::I64(val) => write!(f, "{:?}", val),
            Literal::I128(val) => write!(f, "{:?}", val),
            #[cfg(feature = "unstable-f16")]
            Literal::F16(val) => write!(f, "{:?}", val),
            Literal::F32(val) => write!(f, "{:?}", val),
            Literal::F64(val) => write!(f, "{:?}", val),
            #[cfg(feature = "unstable-f128")]
            Literal::F128(val) => write!(f, "{:?}", val),
            Literal::Bool(val) => write!(f, "{:?}", val),
            Literal::Char(val) => write!(f, "{:?}", val),
            Literal::String(val) => write!(f, "{:?}", val),
        }
    }
}
impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
