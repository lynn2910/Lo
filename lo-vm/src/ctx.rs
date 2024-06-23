use std::collections::HashMap;
use lo_core::function::Function;

#[derive(Debug, Default)]
pub struct Context {
    pub variables: HashMap<String, lo_core::values::Value>,
    pub functions: HashMap<String, Function>
}

pub const SYSTEM_FUNCTIONS: &[&str] = &["println"];

impl Context {
    pub fn is_function_system(&self, name: impl ToString) -> bool {
        dbg!(&self.functions);
        self.functions.get(&name.to_string())
            .map(|f| f.is_system())
            .unwrap_or(false)
    }

    /// This method will add all functions and modules defined at the start :)
    pub fn define_as_global(&mut self) {
        self.functions.insert("println".to_string(), Function::new_system("println"));
    }
}
