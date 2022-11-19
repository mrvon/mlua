//! This example shows a simple lua interpreter.

use mlua::{Lua, MultiValue, StdLib};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("script path not found");
        return;
    }
    let script_path = args[1].clone();
    let lua = unsafe { Lua::unsafe_new() };
    lua.load_from_std_lib(StdLib::DEBUG).unwrap();
    let source = std::fs::read_to_string(script_path).unwrap();
    match lua.load(&source).eval::<MultiValue>() {
        Ok(values) => {
            println!(
                "{}",
                values
                    .iter()
                    .map(|value| format!("{:?}", value))
                    .collect::<Vec<_>>()
                    .join("\t")
            );
        }
        Err(e) => {
            eprintln!("error: {}", e);
        }
    };
}
