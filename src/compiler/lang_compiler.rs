use crate::parser::lang_ast;
use crate::vm::lang_bytecode;

use std::collections::HashMap;

pub struct CompilerArtifacts {
    pub bytecode: Vec<lang_bytecode::Instruction>,
    pub errors: Vec<i32>,
}

struct SymbolTable {
    symbols: HashMap<String, usize>,
    reg_top: usize,
}

impl SymbolTable {
    fn new () -> Self {
        Self {
            symbols: HashMap::new(),
            reg_top: 2,
        }
    }

    fn register_variable (&mut self, name: &str) -> usize {
        self.symbols.insert(name.to_string(), self.reg_top);
        self.reg_top += 1;
        self.reg_top - 1 
    }

    fn get_variable (&self, name: &str) -> Option<&usize> {
        self.symbols.get(name)
    }
}

pub fn compile (ast: &lang_ast::AST) -> CompilerArtifacts {
    let mut symbol_table = SymbolTable::new();
    let compiler_artifacts = CompilerArtifacts {
        bytecode: compile_function(&ast.main, &mut symbol_table),
        errors: Vec::new(),
    };

    compiler_artifacts
}

pub fn compile_function (function: &lang_ast::Function, symbol_table: &mut SymbolTable) 
-> Vec<lang_bytecode::Instruction> {

    let mut bytecode = Vec::new();

    for statement in &function.body {
        bytecode.append(&mut compile_statement(statement, symbol_table));
    }

    bytecode
}

pub fn compile_statement (statement: &lang_ast::Statement, symbol_table: &mut SymbolTable) 
-> Vec<lang_bytecode::Instruction> {
    let mut bytecode = Vec::new();

    use lang_ast::Statement as Statement;
    match statement {
        
        Statement::Function(function) => {

        },
        Statement::DataTypeDef(data_type) => {

        },
        Statement::Expression(expression) => {

        },
        Statement::Assignment(assignment) => {
            bytecode.append(&mut compile_assignment(assignment, symbol_table));
        },
        Statement::Conditional(conditional) => {

        },
        Statement::Loop(loop_statement) => {

        },
        Statement::Return(return_statement) => {

        },
        Statement::Print(symbol) => {
            bytecode.append(&mut compile_print(symbol, symbol_table));
        }
    }

    bytecode

}

pub fn compile_assignment (assignment: &lang_ast::Assignment, symbol_table: &mut SymbolTable) 
-> Vec<lang_bytecode::Instruction> {
    let mut bytecode = Vec::new();
    bytecode.append(&mut compile_expression(&assignment.rhs, symbol_table));
    bytecode.push(
        lang_bytecode::Instruction::Mov(
            symbol_table.register_variable(&assignment.lhs[0].name), 
            0,
        ));

    bytecode
}

pub fn compile_expression (expression: &lang_ast::Expression, symbol_table: &mut SymbolTable) 
-> Vec<lang_bytecode::Instruction> {
    let mut bytecode = Vec::new();

    use lang_ast::Expression as Expression;
    match expression {
        Expression::Constant(constant) => {
            match constant {
                lang_ast::Constant::Integer(integer) => {
                    bytecode.push(lang_bytecode::Instruction::MovConst(0, *integer));
                }
            }
        },
        Expression::FunctionCall => {

        },
        Expression::BinaryExpression(binary_expression) => {
            bytecode.append(&mut compile_binary_expression(binary_expression, symbol_table));
        },
    }

    bytecode
}

pub fn compile_binary_expression (binary_expression: &lang_ast::BinaryExpression, symbol_table: &mut SymbolTable) 
-> Vec<lang_bytecode::Instruction> {
    let mut bytecode = Vec::new();

    bytecode.append(&mut compile_expression(&binary_expression.rhs, symbol_table));

    bytecode.push(lang_bytecode::Instruction::Mov(1, 0));

    bytecode.append(&mut compile_expression(&binary_expression.lhs, symbol_table));
    
    use lang_ast::BinaryOperator as BinaryOperator;
    
    bytecode.push(match binary_expression.operator {
        BinaryOperator::Addition => {
            lang_bytecode::Instruction::Add(0, 1)
        }
        BinaryOperator::Subtraction => {
            lang_bytecode::Instruction::Sub(0, 1)
        }
        BinaryOperator::Multiplication => {
            lang_bytecode::Instruction::Mul(0, 1)
        }
        BinaryOperator::Division => {
            lang_bytecode::Instruction::Div(0, 1)
        }
        BinaryOperator::Mod => {
            lang_bytecode::Instruction::Mod(0, 1)
        }
    });

    bytecode
}

pub fn compile_print (symbol: &lang_ast::Symbol, symbol_table: &mut SymbolTable) 
-> Vec<lang_bytecode::Instruction> {
    let mut bytecode = Vec::new();
    if let Some(register_id) = symbol_table.get_variable(&symbol.name) {
        bytecode.push(lang_bytecode::Instruction::Print(*register_id));
    }
    
    bytecode
}