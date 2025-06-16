use nvim_oxi::{
    self, Dictionary, Function,
    api::{self, opts::OptionOpts},
    mlua::{FromLua, Table, lua},
};

#[nvim_oxi::plugin]
fn wildcat_rust() -> nvim_oxi::Result<Dictionary> {
    let hello = Function::from_fn(|()| {
        //         let m: Table = get_lua_module(Module("require'wildcat.config'.SETTINGS"), Variable("jvm"));
        let m: Table = get_lua_module2("wildcat.config").unwrap();
        let settings: Table = m.get("SETTINGS").unwrap();
        nvim_oxi::api::out_write(format!("{:#?} \n", settings));
    });

    let cmd = Function::from_fn(|()| {
        api::command(
            "sp | resize 15 | start | terminal /home/javier/dev/tomcat/bin/catalina.sh run",
        )
        .unwrap();

        api::command("file wildcat_server_console").unwrap();

        let lualine: nvim_oxi::Result<Table> = get_lua_module2("lualine");

        if let Ok(lualine_table) = lualine {
            let hide_fn: nvim_oxi::mlua::Function = lualine_table.get("hide").unwrap();
            hide_fn.call::<()>(()).unwrap();
        }
        let opts = OptionOpts::builder()
            .scope(api::opts::OptionScope::Local)
            .build();
        api::set_option_value("laststatus", 3, &opts).unwrap();

        let stl = format!("%#Normal# {} Console   {}", "icon", "some.war");
        api::set_option_value("statusline", stl, &opts).unwrap();
    });

    let api = Dictionary::from_iter([("hello", hello), ("cmd", cmd)]);

    Ok(api)
}

pub struct Module<'a>(pub &'a str);
pub struct Variable<'a>(pub &'a str);

pub fn get_lua_module<V: FromLua>(module: Module, variable: Variable) -> V {
    let lua = lua();
    lua.load(format!("{} = {}", variable.0, module.0))
        .exec()
        .unwrap();

    let lua_module: V = lua.globals().get(variable.0).unwrap();
    lua_module
}

pub fn get_lua_module2<V: FromLua>(module_name: &str) -> Result<V, nvim_oxi::Error> {
    let lua = lua();
    let lua_module: V = lua
        .load(format!("return require('{}')", module_name))
        .eval()?;
    Ok(lua_module)
}
