use nvim_oxi::{Dictionary, conversion::FromObject};

use crate::{
    server::{Jboss, Tomcat},
    util,
    wildcat::{Wildcat, WildcatBuilder},
};

pub static mut WILDCAT: Option<Wildcat> = None;

pub fn setup(dictionary: Dictionary) {
    let mut wildcat_builder = WildcatBuilder::new();

    let java_home = dictionary
        .get("java_home")
        .and_then(|obj| String::from_object(obj.clone()).ok());

    match java_home {
        Some(java_home) => {
            wildcat_builder.java_home(java_home);
        }
        None => {
            if std::env::var("JAVA_HOME").is_err() {
                util::print_error(
                    "Environment variable JAVA_HOME or java_home in wildcat setup is required \n",
                );
                return;
            }
        }
    }

    if let Some(obj) = dictionary.get("console_size") {
        if let Ok(console_size) = usize::from_object(obj.clone()) {
            wildcat_builder.console_size(console_size);
        }
    }

    if let Some(obj) = dictionary.get("default_server") {
        if let Ok(default_server) = String::from_object(obj.clone()) {
            wildcat_builder.default_server(default_server.into());
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
                util::print_error(e);
                return;
            }
        }
    } else {
        wildcat_builder.tomcat(Tomcat::default());
    }

    if let Some(obj) = dictionary.get("jboss") {
        match Jboss::from_object(obj.clone()) {
            Ok(jboss) => {
                wildcat_builder.jboss(jboss);
            }
            Err(e) => {
                util::print_error(e);
                return;
            }
        }
    } else {
        wildcat_builder.jboss(Jboss::default());
    }

    let wildcat = wildcat_builder.build();

    unsafe {
        WILDCAT = Some(wildcat);
    }
}

pub fn switch(_: ()) {
    unsafe {
        if let Some(ref mut wildcat) = WILDCAT {
            wildcat.switch();
        }
    }
}

pub fn clean(_: ()) {
    unsafe {
        if let Some(ref wildcat) = WILDCAT {
            wildcat.clean();
        }
    }
}

pub fn deploy(from: String) {
    unsafe {
        if let Some(ref wildcat) = WILDCAT {
            match wildcat.deploy(&from) {
                Ok(_) => util::print_info("Done!"),
                Err(e) => util::print_error(e),
            }
        }
    }
}

pub fn run(dir: String) {
    unsafe {
        if let Some(ref wildcat) = WILDCAT {
            if let Err(e) = wildcat.run(&dir) {
                util::print_error(e);
            }
        }
    }
}

pub fn up(_: ()) {
    unsafe {
        if let Some(ref wildcat) = WILDCAT {
            if let Err(e) = wildcat.up() {
                util::print_error(e);
            }
        }
    }
}

pub fn down(_: ()) {
    let _ = nvim_oxi::api::command("bd! wildcat_server_console");
}

pub fn get_default_server(_: ()) -> String {
    unsafe {
        if let Some(ref wildcat) = WILDCAT {
            wildcat.get_default_server_as_str()
        } else {
            "No Info".to_string()
        }
    }
}

pub fn get_tomcat_info(_: ()) -> (String, String) {
    unsafe {
        if let Some(ref wildcat) = WILDCAT {
            if let Some(info) = wildcat.get_tomcat_info() {
                (info.0, info.1)
            } else {
                (String::new(), String::new())
            }
        } else {
            (String::new(), String::new())
        }
    }
}

pub fn get_jboss_info(_: ()) -> (String, String) {
    unsafe {
        if let Some(ref wildcat) = WILDCAT {
            if let Some(info) = wildcat.get_jboss_info() {
                (info.0, info.1)
            } else {
                (String::new(), String::new())
            }
        } else {
            (String::new(), String::new())
        }
    }
}
