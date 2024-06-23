use std::collections::HashMap;
use lo_core::function::Function;

#[derive(Debug, Default)]
pub struct Context<'a> {
    pub local_variables: HashMap<String, lo_core::values::Value>,
    pub functions: HashMap<String, Function>,
    pub global_ctx: Option<&'a mut Context<'a>>
}

pub const SYSTEM_FUNCTIONS: &[&str] = &["println"];

impl<'a> Context<'a> {
    pub fn is_function_system(&self, name: impl ToString) -> bool {
        self.functions.get(&name.to_string())
            .map(|f| f.is_system())
            .unwrap_or(
                self.global_ctx.as_ref()
                    .map(|c| c.is_function_system(&name.to_string()))
                    .unwrap_or(false)
            )
    }

    pub fn set_global_ctx(&mut self, global: &'a mut Self) {
        self.global_ctx = Some(global);
    }

    pub fn clear_global_ctx(&mut self) {
        self.global_ctx = None;
    }

    /// This method will add all functions and modules defined at the start :)
    pub fn define_as_global(&mut self) {
        self.functions.insert("println".to_string(), Function::new_system("println", "println"));
    }
}