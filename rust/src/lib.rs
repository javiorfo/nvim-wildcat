use std::sync::OnceLock;

use nvim_oxi::{
    self, Dictionary, Function, Object,
    api::{self, opts::OptionOpts},
    conversion::FromObject,
    mlua::{FromLua, Table, lua},
};

use crate::config::{Jboss, Server, Tomcat, Wildcat, WildcatBuilder};

mod config;
mod util;

static WILDCAT: OnceLock<Wildcat> = OnceLock::new();

#[nvim_oxi::plugin]
fn wildcatr() -> nvim_oxi::Result<Dictionary> {
    let hello = Function::from_fn(|()| {
        let m: Table = util::get_lua_module("wildcat.config").unwrap();
        let settings: Table = m.get("SETTINGS").unwrap();
        nvim_oxi::api::out_write(format!("{:#?} \n", settings));
    });

    let cmd = Function::from_fn(|()| {
        api::command(
            "sp | resize 15 | start | terminal /home/javier/dev/tomcat/bin/catalina.sh run",
        )
        .unwrap();

        api::command("file wildcat_server_console").unwrap();

        let lualine: nvim_oxi::Result<Table> = util::get_lua_module("lualine");

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

    let setup = Function::from_fn(|dictionary: Dictionary| {
        let jvm = match dictionary.get("jvm") {
            Some(obj) => String::from_object(obj.clone()).ok(),
            _ => std::env::var("JAVA_HOME").ok(),
        };

        if jvm.is_none() {
            nvim_oxi::api::err_write("[ERROR] JVM not set \n");
            return;
        }

        let mut wildcat_builder = WildcatBuilder::new(jvm.unwrap());

        if let Some(obj) = dictionary.get("console_size") {
            if let Ok(console_size) = usize::from_object(obj.clone()) {
                wildcat_builder.console_size(console_size);
            }
        }

        if let Some(obj) = dictionary.get("default") {
            if let Ok(default) = String::from_object(obj.clone()) {
                wildcat_builder.default_server(default.into());
            }
        }

        if let Some(obj) = dictionary.get("build_tool") {
            if let Ok(build_tool) = String::from_object(obj.clone()) {
                wildcat_builder.build_tool(build_tool.into());
            }
        }

        if let Some(obj) = dictionary.get("tomcat") {
            match Tomcat::from_object(obj.clone()) {
                Ok(tomcat) => {
                    wildcat_builder.tomcat(tomcat);
                }
                Err(e) => {
                    nvim_oxi::api::err_write(&format!("[ERROR] {} \n", e));
                    return;
                }
            }
        }

        if let Some(obj) = dictionary.get("jboss") {
            match Jboss::from_object(obj.clone()) {
                Ok(jboss) => {
                    wildcat_builder.jboss(jboss);
                }
                Err(e) => {
                    nvim_oxi::api::err_write(&format!("[ERROR] {} \n", e));
                    return;
                }
            }
        }

        let wildcat = wildcat_builder.build();

        WILDCAT.set(wildcat).unwrap();

        nvim_oxi::api::out_write(format!("{:#?} \n", WILDCAT));
    });

    let api = Dictionary::from_iter([
        ("hello", Object::from(hello)),
        ("cmd", Object::from(cmd)),
        ("se", Object::from(setup)),
    ]);

    Ok(api)
}
