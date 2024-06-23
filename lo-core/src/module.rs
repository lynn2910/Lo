use std::collections::HashMap;
use crate::ast::InstructionsList;
use crate::function::Function;
use crate::values::Value;

/// Represent a Module
pub struct Module {
    /// The ID of the module;
    pub id: String,
    /// The functions declared
    pub functions: HashMap<String, Function>,
    /// All constants created
    pub constants: HashMap<String, Value>,
    /// This code is what will be immediately executed; it can be represented as the code executed from the start or the code in an if block
    pub unscoped_instructions: InstructionsList,
    /// Modules created in the same code
    pub modules: HashMap<String, Box<Module>>
}

impl Module {
    pub fn new_empty(id: impl ToString, unscoped_instructions: InstructionsList) -> Self {
        Module {
            id: id.to_string(),
            unscoped_instructions,
            functions: HashMap::new(),
            constants: HashMap::new(),
            modules: HashMap::new()
        }
    }

    pub fn add_constant(&mut self, name: String, v: Value) {
        if self.constants.contains_key(&name) { panic!("Constant {name} already defined"); }
        self.constants.insert(name, v);
    }
    
    pub fn add_function(&mut self, name: String, function: Function) {
        if self.functions.contains_key(&name) { panic!("Function {name} already defined"); }
        self.functions.insert(name, function);
    }
    
    pub fn add_module(&mut self, name: String, module: Module) {
        if self.modules.contains_key(&name) { panic!("Module {name} already defined"); }
        self.modules.insert(name, Box::new(module));
    }
}