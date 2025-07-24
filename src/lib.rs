use nvim_oxi::{
    self, Dictionary, Function, Object,
    api::{
        self,
        opts::{CreateAugroupOpts, CreateAutocmdOptsBuilder},
    },
    mlua,
};

mod core;
mod error;
mod server;
mod util;
mod wildcat;

#[nvim_oxi::plugin]
fn wildcat() -> nvim_oxi::Result<Dictionary> {
    let ag_name = "wildcat_statusline";

    api::create_augroup(ag_name, &CreateAugroupOpts::builder().clear(true).build())?;

    api::create_autocmd(
        ["BufDelete"],
        &CreateAutocmdOptsBuilder::default()
            .group(ag_name)
            .patterns(vec!["wildcat_server_console"])
            .callback(|_| {
                if let Ok(lualine_table) = util::get_lua_module::<mlua::Table>("lualine") {
                    if let Ok(fun) = lualine_table.get::<mlua::Function>("hide") {
                        let _ = mlua::lua().create_table().and_then(|table| {
                            table.set("unhide", true).unwrap();
                            fun.call::<()>(table)
                        });
                    }
                }
                true
            })
            .build(),
    )?;

    unsafe {
        core::WILDCAT = Some(wildcat::Wildcat::default());
    }

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
