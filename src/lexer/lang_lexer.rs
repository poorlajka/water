use crate::lang_token::{Token, LexingError};
use logos::{Logos, Span};
use chumsky::span::{SimpleSpan};


fn count_columns(s: &str) -> usize {
    let mut column = 0;
    for c in s.chars() {
        match c {
            ' ' => column += 1,
            '\t' => column = (column / 8 + 1) * 8,
            _ => break,
        }
    }
    column
}

pub struct LexerArtifacts<'a> {
    pub tokens: Vec<Token<'a>>,
    pub errors: Vec<LexingError>,
}

pub fn tokenize (code: &str) -> Vec<(Token, SimpleSpan)> {
    let mut lexer_artifacts = LexerArtifacts {
        tokens: Vec::new(),
        errors: Vec::new(),
    };
    let mut tokens = Vec::new();
    let mut lexer = Token::lexer(code).spanned();
    
    let mut old_indent = 0;

    while let Some((token, span)) = lexer.next() {
        match token.clone() {
            Ok(token) => tokens.push((token, span.clone().into())),
            Err(error) => tokens.push((Token::Error, span.clone().into())),
        }

        if let Ok(Token::Newline) = token {
            let slice = lexer.remainder();
            
            let leading_ws: String = slice.chars().take_while(|c| *c == ' ' || *c == '\t').collect();
            let new_indent = count_columns(&leading_ws);
            
            if new_indent > old_indent {
                tokens.push((Token::Indent, span.clone().into()));
            }
            else if new_indent < old_indent {
                tokens.push((Token::Dedent, span.clone().into()));
            }
            old_indent = new_indent;
        }
    }

    tokens
}