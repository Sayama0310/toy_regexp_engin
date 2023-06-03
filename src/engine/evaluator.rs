//! Evaluator Module
//!
//! This module contains the implementation of the regular expression evaluator.
use crate::engine::compiler::Instruction;

pub fn evaluate(instructions: Vec<Instruction>, line: &str) -> Result<bool, EvaluateError> {
    let mut evaluator = Evaluator::new(instructions, line.chars().collect::<Vec<char>>());
    evaluator.evaluate()
}

/// Evaluator
struct Evaluator {
    instructions: Vec<Instruction>,
    line: Vec<char>,
    pc: usize,
    sp: usize,
    context: Vec<(usize, usize)>,
}

impl Evaluator {
    fn new(instructions: Vec<Instruction>, line: Vec<char>) -> Evaluator {
        Evaluator {
            instructions,
            line,
            pc: 0,
            sp: 0,
            context: Vec::new(),
        }
    }

    fn evaluate(&mut self) -> Result<bool, EvaluateError> {
        // TODO Implement the evaluator.
        Ok(true)
    }
}

/// Evaluate Error
#[derive(Debug)]
pub enum EvaluateError {
    Unimplemented,
}

impl std::fmt::Display for EvaluateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluateError::Unimplemented => write!(f, "Unimplemented"),
        }
    }
}

impl std::error::Error for EvaluateError {}
