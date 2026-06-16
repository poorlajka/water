pub mod bytecode;
pub mod codegen;
pub mod linker;
pub mod vm;

use crate::frontend::FrontendOutput;
use crate::backend::bytecode::Program;

pub fn compile(frontend: FrontendOutput) -> Program {
    let bytecode_modules = frontend.modules.iter()
        .map(|(path, ast)| (path.clone(), codegen::compile_module_from_ast(ast)))
        .collect();
    linker::link(bytecode_modules)
}
