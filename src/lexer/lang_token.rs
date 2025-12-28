
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexingError)]
pub enum Token<'a> {

    // Keywords
    #[token("for")]
    For,

    #[token("let")]
    Let,

    #[token("in")]
    In,

    #[token("fn")]
    Function,

    #[token("type")]
    Type,

    #[token("print")]
    Print,

    #[token("interface")]
    Interface,

    #[token("\n")]
    #[token(";")]
    Newline,
    
    #[token("=")]
    Assignment,

    // Identifiers
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice())]
    Identifier(&'a str),

    #[regex("-?[0-9]+", |lex| lex.slice().parse())]
    Integer(i64),

    #[regex("-?[-1-9]+\\.[0-9]+", |lex| lex.slice().parse())]
    Float(f64),

    // Punctuation
    #[token(":")]
    Colon,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,

    #[token("(")]
    Lparen,

    #[token(")")]
    Rparen,

    #[token("[")]
    Lbracket,

    #[token("]")]
    Rbracket,

    #[token("/")]
    Slash,

    #[token("..")]
    Range,

    // Whitespace (all spaces/tabs)
    #[regex(r"[ \t]+", logos::skip)]
    Whitespace,

    Indent,
    Dedent,

    Error,

    Eof,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum LexingError {
    NumberParseError,
    #[default]
    Other
}

impl From<std::num::ParseIntError> for LexingError {
   fn from(_: std::num::ParseIntError) -> Self {
      LexingError::NumberParseError
  }
}

impl From<std::num::ParseFloatError> for LexingError {
  fn from(_: std::num::ParseFloatError) -> Self {
     LexingError::NumberParseError
  }
}