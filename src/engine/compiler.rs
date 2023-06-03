//! Compiler Module
//!
//! This module contains the implementation of the regular expression instruction compiler.
use std::error::Error;
use crate::engine::parser::AST;

pub fn compile(ast: &AST) -> Result<Vec<Instruction>, CompileError> {
    let mut compiler = Compiler::new();
    compiler.compile(ast)
}

/// Compiler
struct Compiler {
    instructions: Vec<Instruction>,
}

impl Compiler {
    fn new() -> Compiler {
        Compiler {
            instructions: Vec::new(),
        }
    }

    fn compile(&mut self, ast: &AST) -> Result<Vec<Instruction>, CompileError> {
        // TODO Implement the compiler.
        Ok(Vec::new())
    }
}

/// Instruction
#[derive(Debug, PartialEq)]
pub enum Instruction {
    Char(char),
    Match,
    Jump(usize),
    Split(usize, usize),
}

/// Compile Error
#[derive(Debug)]
pub enum CompileError {
    Unimplemented,
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::Unimplemented => write!(f, "Unimplemented"),
        }
    }
}

impl Error for CompileError {}