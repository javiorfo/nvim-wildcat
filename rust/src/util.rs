use nvim_oxi::mlua::{lua, FromLua};


pub fn get_lua_module<V: FromLua>(module_name: &str) -> Result<V, nvim_oxi::Error> {
    let lua = lua();
    let lua_module: V = lua
        .load(format!("return require('{}')", module_name))
        .eval()?;
    Ok(lua_module)
}

pub struct Module<'a>(pub &'a str);
pub struct Variable<'a>(pub &'a str);

pub fn get_lua_module2<V: FromLua>(module: Module, variable: Variable) -> V {
    let lua = lua();
    lua.load(format!("{} = {}", variable.0, module.0))
        .exec()
        .unwrap();

    let lua_module: V = lua.globals().get(variable.0).unwrap();
    lua_module
}
