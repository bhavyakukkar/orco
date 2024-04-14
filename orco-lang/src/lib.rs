//! OrCo language, IR for OrCo compiler toolchain
#![warn(missing_docs)]

use lalrpop_util::lalrpop_mod;

/// Lexer (splits input into tokens)
pub mod lexer;

pub(crate) mod parser_utils;
lalrpop_mod!(#[allow(missing_docs)] pub parser);

/// A compilation unit
pub struct Crate {
    pub root: orco::ir::Module,
}

impl Crate {
    /// Parse the crate
    pub fn parse(path: impl AsRef<std::path::Path>) -> Self {
        Self {
            root: parser::ModuleParser::new()
                .parse(lexer::Lexer::new(&std::fs::read_to_string(path).unwrap()))
                .unwrap(),
        }
    }
}
