pub mod ast;
pub mod lexer;
pub mod parser;
pub mod diagnostics;
pub mod type_checker;

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::frontend::ast::{Module, Statement};
use crate::frontend::type_checker::TypeMap;
use crate::frontend::diagnostics::{Diagnostic, Severity};

pub struct FrontendOutput {
    pub modules: Vec<(String, Module)>,
    pub type_map: TypeMap,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn compile(source: &str, base_dir: Option<&Path>) -> FrontendOutput {
    let mut diagnostics = Vec::new();
    let modules = parse_modules(source, base_dir, &mut diagnostics);
    let type_map = type_checker::check(&modules);
    FrontendOutput { modules, type_map, diagnostics }
}

fn parse_modules(source: &str, base_dir: Option<&Path>, diagnostics: &mut Vec<Diagnostic>) -> Vec<(String, Module)> {
    let mut ordered = Vec::new();
    let mut visited = HashSet::new();
    visited.insert("__main__".to_string());

    let main_ast = parse_module(source, "__main__", diagnostics);
    parse_imports(&main_ast, base_dir, &mut ordered, &mut visited, diagnostics);
    ordered.push(("__main__".to_string(), main_ast));
    ordered
}

fn parse_imports(
    ast: &Module,
    base_dir: Option<&Path>,
    ordered: &mut Vec<(String, Module)>,
    visited: &mut HashSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for stmt in &ast.statements {
        let imp_path = match stmt {
            Statement::ImportFrom { path: p, .. } => Some(p.as_str()),
            Statement::ImportModule { path: p, .. } => Some(p.as_str()),
            _ => None,
        };
        if let Some(imp_path) = imp_path {
            if !visited.insert(imp_path.to_string()) { continue; }

            let file_path = module_file_path(imp_path, base_dir);
            let source = fs::read_to_string(&file_path)
                .unwrap_or_else(|_| panic!("cannot load module '{}'", imp_path));

            let dep_ast = parse_module(&source, imp_path, diagnostics);
            let dep_base = file_path.parent().map(|p| p.to_path_buf());
            parse_imports(&dep_ast, dep_base.as_deref(), ordered, visited, diagnostics);
            ordered.push((imp_path.to_string(), dep_ast));
        }
    }
}

fn parse_module(source: &str, name: &str, diagnostics: &mut Vec<Diagnostic>) -> Module {
    let lexer::LexingArtifacts { tokens, .. } = lexer::tokenize(source);
    let artifacts = parser::parse_module(&tokens, &name.to_string());
    for error in artifacts.errors {
        diagnostics.push(Diagnostic {
            severity: Severity::Error,
            message: error.message,
            labels: error.span.map(|s| vec![diagnostics::Label { span: s, message: None }]).unwrap_or_default(),
        });
    }
    artifacts.ast
}

fn module_file_path(path: &str, base_dir: Option<&Path>) -> PathBuf {
    let mut p = base_dir.map(|d| d.join(path)).unwrap_or_else(|| PathBuf::from(path));
    p.set_extension("water");
    p
}
