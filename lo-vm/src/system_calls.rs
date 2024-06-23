use lo_core::values::Value;

pub fn sys_println(args: Vec<Value>) {
    for arg in args {
        println!("{arg}");
    }
}