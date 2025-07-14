use std::fmt::Display;

use nvim_oxi::mlua::{FromLua, lua};

pub fn get_lua_module<T: FromLua>(module_name: &str) -> Result<T, nvim_oxi::Error> {
    let lua = lua();
    let lua_module: T = lua
        .load(format!("return require('{module_name}')"))
        .eval()?;
    Ok(lua_module)
}

pub fn print_error<T: Display>(msg: T) {
    nvim_oxi::api::err_write(&format!("󰄛  Wildcat   [ERROR] {msg} \n"));
}

pub fn print_info<T: Display>(msg: T) {
    nvim_oxi::api::out_write(format!("󰄛  Wildcat   {msg} \n"));
}
