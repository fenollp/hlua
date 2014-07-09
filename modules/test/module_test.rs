#![crate_type = "dylib"]
#![feature(phase)]

#[phase(plugin)]
extern crate rust_hl_lua_modules;

#[export_lua_module]
pub mod mylib {
    static PI:f32 = 3.141592;

    fn function1(a: int, b: int) -> int { a + b }

    fn function2(a: int) -> int { a + 5 }
}
