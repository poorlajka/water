mod diagnostics;

use std::fs;
use std::path::Path;

fn main() {
    let path = "examples/example.txt";
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => { eprintln!("Failed to read program: {}", e); return; }
    };

    let base_dir = Path::new(path).parent();
    let frontend_output = water::frontend::compile(&source, base_dir);
    diagnostics::emit_diagnostics(&source, &frontend_output.diagnostics);

    let program = water::backend::compile(frontend_output);
    water::backend::vm::exec(&program);
}
