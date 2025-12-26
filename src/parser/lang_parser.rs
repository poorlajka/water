use crate::lexer::lang_token::Token;
use crate::parser::lang_ast;

pub struct ParserArtifacts {
    pub ast: lang_ast::AST,
    pub errors: i32,
}

pub fn parse (tokens: &Vec<Token>) -> ParserArtifacts {

    let mut ast = lang_ast::AST {
        main: lang_ast::Function {
            function_decl: lang_ast::FunctionDecl {
                name: lang_ast::Symbol { name: "main".to_string() },
                inputs: vec![],
                outputs: vec![],
            },
            body: parse_function_body(tokens, 0).0,
        }
    };

    ParserArtifacts {
        ast,
        errors: 5,
    }
}

fn parse_function_body (tokens: &Vec<Token>, mut curr_token: usize) 
-> (Vec<lang_ast::Statement>, usize) {
    
    let mut function_body = Vec::<lang_ast::Statement>::new();

    while curr_token < tokens.len() {
        match tokens[curr_token] {
            Token::Function => {
                let (function, new_curr_token) = parse_function(tokens, curr_token);
                curr_token = new_curr_token;
                function_body.push(lang_ast::Statement::Function(function));
            }
            Token::Let => {
                let (assignment, new_curr_token) = parse_assignment(tokens, curr_token);
                curr_token = new_curr_token;
                function_body.push(lang_ast::Statement::Assignment(assignment));
                
            }
            _ => {
                curr_token+=1;
            }
        }
    }

    (function_body, curr_token)
}

fn parse_function (tokens: &Vec<Token>, mut curr_token:  usize) 
-> (lang_ast::Function, usize) {

    let (function_decl, curr_token) = parse_function_decl(tokens, curr_token);
    let (body, curr_token) = parse_function_body(tokens, curr_token);
    
    (
        lang_ast::Function {
            function_decl,
            body,
        },
        curr_token,
    )
}

fn parse_function_decl (tokens: &Vec<Token>, curr_token: usize) 
-> (lang_ast::FunctionDecl, usize) {

    (
        lang_ast::FunctionDecl {
            name: lang_ast::Symbol { name: "fn".to_string() },
            inputs: Vec::new(),
            outputs: Vec::new(),
        }, 
        4
    )
}

fn parse_assignment (tokens: &Vec<Token>, mut curr_token:  usize) 
-> (lang_ast::Assignment, usize) {

    curr_token += 1;
    let (lhs, mut curr_token) = parse_assignment_lhs(tokens, curr_token);
    
    if !matches!(tokens[curr_token], Token::Assignment) {

    }
    curr_token += 1;
    
    let (rhs, curr_token) = parse_assignment_rhs(tokens, curr_token);
    
    (
        lang_ast::Assignment {
            lhs,
            rhs,
        },
        curr_token,
    )
}

fn parse_assignment_lhs (tokens: &Vec<Token>, mut curr_token:  usize) 
-> (Vec<lang_ast::Symbol>, usize) {

    let mut lhs = Vec::new();
    while !matches!(tokens[curr_token], Token::Assignment) {
        if let Token::Identifier(identifier) = &tokens[curr_token] {
            lhs.push(lang_ast::Symbol {
                name: identifier.to_string(),
            })
        }
        else {
            // Parser error
        }
        curr_token += 1;
    }
    
    (lhs, curr_token)
}

fn parse_assignment_rhs (tokens: &Vec<Token>, mut curr_token:  usize) 
-> (lang_ast::Expression, usize) {

    parse_expression(tokens, curr_token)
}

fn parse_expression (tokens: &Vec<Token>, mut curr_token:  usize) 
-> (lang_ast::Expression, usize) {
    
    let expression = match tokens[curr_token] {
        Token::Integer(integer) => {
            lang_ast::Expression::Constant(
                lang_ast::Constant::Integer(integer)
            )
        }
        _ => {
            lang_ast::Expression::Constant(
                lang_ast::Constant::Integer(5)
            )
        }
    };
    curr_token += 1;
    (expression, curr_token)
}