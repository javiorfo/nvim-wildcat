use nvim_oxi::{
    self, Dictionary, Function, Object,
    api::{self, opts::CreateAutocmdOptsBuilder},
    mlua,
};

mod core;
mod error;
mod server;
mod util;
mod wildcat;

#[nvim_oxi::plugin]
fn wildcat() -> nvim_oxi::Result<Dictionary> {
    api::create_autocmd(
        vec!["BufDelete"],
        &CreateAutocmdOptsBuilder::default()
            .patterns(vec!["wildcat_server_console"])
            .callback(|_| {
                if let Ok(lualine_table) = util::get_lua_module::<mlua::Table>("lualine") {
                    let hide_fn: mlua::Function = lualine_table.get("hide").unwrap();
                    let table = mlua::lua().create_table().unwrap();
                    table.set("unhide", true).unwrap();
                    hide_fn.call::<()>(table).unwrap();
                }
                true
            })
            .build(),
    )?;

    let api = Dictionary::from_iter([
        ("clean", Object::from(Function::from_fn(core::clean))),
        ("deploy", Object::from(Function::from_fn(core::deploy))),
        ("down", Object::from(Function::from_fn(core::down))),
        (
            "get_default_server",
            Object::from(Function::from_fn(core::get_default_server)),
        ),
        (
            "get_tomcat_info",
            Object::from(Function::from_fn(core::get_tomcat_info)),
        ),
        (
            "get_jboss_info",
            Object::from(Function::from_fn(core::get_jboss_info)),
        ),
        ("run", Object::from(Function::from_fn(core::run))),
        ("setup", Object::from(Function::from_fn(core::setup))),
        ("switch", Object::from(Function::from_fn(core::switch))),
        ("up", Object::from(Function::from_fn(core::up))),
    ]);

    Ok(api)
}
