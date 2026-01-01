use crate::lexer::lang_token::Token;
use crate::parser::lang_ast;

use chumsky::{
    input::{Stream, ValueInput},
    prelude::*,
};

use chumsky::input::IterInput;


pub struct ParserArtifacts {
    pub ast: lang_ast::AST,
    pub errors: i32,
}

/*
    Holy shit there has to be a better way of writing this right????
*/
pub fn parser<'tokens, 'src: 'tokens, I>(
) -> impl Parser<
    'tokens,
    I,
    lang_ast::AST,
    extra::Err<Rich<'tokens, (Token<'src>, SimpleSpan)>>
>
where
    I: ValueInput<
        'tokens,
        Token = (Token<'src>, SimpleSpan),
        Span = SimpleSpan,
    >,
{

    /*
        <unary_operator> ::= "-" | "not"
        <binary_operator> ::= 
            "+" | "-" | "*" | "/" | "%"
            | "and" | "or" | "==" | "!=" 
            | "<" | "<=" | ">" | ">="
    */

    /*
        <function_call> ::= IDENTIFIER "(" [ <expression> ( "," <expression> )* [ "," ] ] ")"
    */

    /*
        <unaryexpression> ::= <unary_operator>* ( <function_call> | CONSTANT | IDENTIFIER )
        <expression> ::= <unaryexpression> [ <binary_operator> <expression> ]*
    */

    /*
        <loop> ::= <for_loop> | <while_loop>
    */

    /*
        <branch> ::= <expression> ":" <indent> (<statement>)* <dedent>
        <conditional> ::= "if" <branch> ( "else" "if" <branch> )* [ "else" <branch> ]
    */

    /*
        <assignment> ::= [ "mut" ] IDENTIFIER ( "," [ "mut" ] IDENTIFIER )* "=" <expression>
    */

    /*
        <statement> ::= <expression> 
            | <assignment> 
            | <conditional> 
            | <loop> 
            | <function> 
            | <datatype>
    */

    /*
        <function_body> ::= <statement>* [ <function_return> ]
    */
    let function_body = statement()
        .repeated()
        .collect::<Vec<lang_ast::Statement>>();

    /*
        <param> ::= IDENTIFIER [ ":" <type> ]
        <param_list> ::= "(" [ <param> ( "," <param> )* [ "," ] ] ")"
    */

    /*
        <function_decl> ::= "fn" IDENTIFIER <param_list> "->" <type> ":"
    */
    let function_decl = just(lang_token::Function)
        .ignore_then(parameter()
            .repeated()
            .delimited_by(just('('), just(')')))
        .map(lang_ast::FunctionDecl {

        })

    /*
        <function> ::= <function_decl> <indent> <function_body> <dedent>
    */
    let function = function_decl()
        .then(function_body())
        .map(|decl, body| lang_ast::Function {
            function_decl: decl,
            body: body,
        });

    any()
        .map(|s| {
            lang_ast::AST {
                main: lang_ast::Function {
                    function_decl: lang_ast::FunctionDecl {
                        name: lang_ast::Symbol { name: "main".to_string() },
                        inputs: vec![],
                        outputs: vec![],
                    },
                    body: vec![],
                }
            }
        })
}
/*
pub fn parse (tokens: &Vec<Token>) -> ParserArtifacts {

    
    let mut ast = lang_ast::AST {
        main: lang_ast::Function {
            function_decl: lang_ast::FunctionDecl {
                name: lang_ast::Symbol { name: "main".to_string() },
                inputs: vec![],
                outputs: vec![],
            },
            body: function().,
        }
    };

    ParserArtifacts {
        ast,
        errors: 5,
    }
}

fn function<'a>() -> Parser<'a, Function> {
    function_decl()
        .then(function_body())
        .map(|(function_decl, function_body)| Function { function_decl, function_body })
        .boxed()
}

fn function_body<'a>() -> Parser<'a, Vec<Statement> {
    statement()
        .repeated()
        .delimited_by(just(Token::LBrace), just(Token::RBrace))
        .boxed()
}

fn statement<'a>() -> Parser<'a, Statement> {

}
*/

/*

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
            Token::Print => {
                curr_token += 1;
                if let Token::Identifier(identifier) = &tokens[curr_token] {
                    function_body.push(lang_ast::Statement::Print(
                        lang_ast::Symbol { name: identifier.to_string() },
                    ));
                }
                else {
                    // Parser error
                }
                curr_token += 1;
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
    
    let mut sub_expressions = Vec::new();

    use lang_lexer::Token as Token;
    while !matches!(tokens[curr_token], Token::Newline) {
        match tokens[curr_token] {

            /* 
                Identifier(expression)
                | identifier[expression]
                | identifier
            */
            Token::Identifier(identifier) => {
                /* 
                    Function call
                */
                if matches!(tokens[curr_token+1], Token::Lparen) {
                    let (sub_expression, new_curr_token) = parse_fn_call(tokens, curr_token);
                    curr_token = new_curr_token;
                    sub_expressions.push(sub_expression);
                }
                /* 
                    Container indexing
                */
                else if matches!(tokens[curr_token+1],) {
                    let (sub_expression, new_curr_token) = parse_index(tokens, curr_token);
                    curr_token = new_curr_token;
                    sub_expressions.push(sub_expression);
                }
                /* 
                    Variable
                */
                else {
                    sub_expressions.push(lang_ast::Variable(identifier));
                    curr_token+=1;
                }
            }
            Token::Integer(integer) => {
                sub_expressions.push(
                    lang_ast::Expression::Constant(
                        lang_ast::Constant::Integer(integer)
                    )
                );
                curr_token+=1;
            }
            Token::Float(float) => {
                sub_expressions.push(
                    lang_ast::Expression::Constant(
                        lang_ast::Constant::Float(float)
                    )
                );
                curr_token+=1;
            }

            Token::Plus => {
                let rhs, new_curr_token = parse_expression
                curr_token
                return lang_ast::BinaryExpression {
                    lhs: sub_expression[0],
                    rhs
                }
            }

            _ => {

            }
        }
    }
    

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
*/