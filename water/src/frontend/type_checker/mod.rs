use crate::frontend::ast::Module;
use std::collections::HashMap;

pub type TypeMap = HashMap<usize, crate::frontend::ast::Type>;

pub fn check(_modules: &[(String, Module)]) -> TypeMap {
    TypeMap::new()
}
