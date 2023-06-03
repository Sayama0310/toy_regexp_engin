//! Parser Module
//!
//! This module contains the implementation of the regular expression parser.
pub fn parse(regexp: &str) -> Result<AST, ParseError> {
    let mut parser = Parser::new(regexp);
    parser.parse()
}

/// Parser
struct Parser<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl Parser<'_> {
    fn new(regexp: &str) -> Parser {
        Parser {
            chars: regexp.chars().peekable(),
        }
    }

    fn parse(&mut self) -> Result<AST, ParseError> {
        // TODO Implement the parser.
        Ok(AST::Dot)
    }
}

/// Abstract Syntax Tree
#[derive(Debug, PartialEq)]
pub enum AST {
    Concat(Vec<AST>),
    Union(Vec<AST>),
    Star(Box<AST>),
    Plus(Box<AST>),
    Question(Box<AST>),
    Char(char),
    Dot,
}

/// Parse Error
#[derive(Debug)]
pub enum ParseError {
    UnexpectedCharacter(char),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedCharacter(c) => {
                write!(f, "Unexpected character: {}", c)
            }
        }
    }
}

impl std::error::Error for ParseError {}
