use lo_core::values::Value;

pub fn sys_fn_called(id: &String, args: Vec<Value>) {
    match id.as_str() { 
        "println" => sys_println(args),
        _ => {}
    }
}

pub fn sys_println(args: Vec<Value>) {
    for arg in args {
        println!("{arg}");
    }
}