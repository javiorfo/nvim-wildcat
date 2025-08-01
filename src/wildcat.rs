use std::{
    fmt::Display,
    io::Read,
    process::{Command, Stdio},
};

use nvim_oxi::{
    api,
    mlua::{self, Table},
};

use crate::{
    error::{Error, Result},
    server::{Jboss, Server, Tomcat},
    util,
};

#[derive(Debug)]
pub struct Wildcat {
    console_size: usize,
    java_home: Option<String>,
    build_tool: BuildTool,
    default_server: Server,
    jboss: Option<Jboss>,
    tomcat: Option<Tomcat>,
}

impl Default for Wildcat {
    fn default() -> Self {
        Self {
            console_size: 15,
            java_home: None,
            build_tool: BuildTool::Maven,
            default_server: Server::Jboss,
            jboss: Some(Jboss::default()),
            tomcat: Some(Tomcat::default()),
        }
    }
}

impl Wildcat {
    pub fn switch(&mut self) {
        match self.default_server {
            Server::Jboss => self.default_server = Server::Tomcat,
            Server::Tomcat => self.default_server = Server::Jboss,
        }
        util::print_info(format!("{} is now enabled", self.default_server.label()));
    }

    pub fn up(&self) -> Result {
        let (run, deploy_list) = match self.default_server {
            Server::Jboss => {
                let jboss = self
                    .jboss
                    .as_ref()
                    .ok_or(Error::Msg("JBoss is not set".to_string()))?;
                (jboss.run_path(), jboss.deployed_as_str())
            }
            Server::Tomcat => {
                let tomcat = self
                    .tomcat
                    .as_ref()
                    .ok_or(Error::Msg("Tomcat is not set".to_string()))?;
                (tomcat.run_path(), tomcat.deployed_as_str())
            }
        };

        let deploy_list = match deploy_list {
            Ok(list) => list,
            Err(e) => {
                return Err(Error::Msg(e.to_string()));
            }
        };

        if let Err(e) = api::command(&format!(
            "sp | resize {} | start | terminal {} {}",
            self.console_size,
            self.set_java_home(),
            run
        )) {
            return Err(Error::Msg(e.to_string()));
        }

        api::command("file wildcat_server_console").unwrap();

        if let Ok(lualine) = util::get_lua_module::<Table>("lualine") {
            let _ = lualine
                .get::<mlua::Function>("hide")
                .and_then(|f| f.call::<()>(()));
        }

        let opts = api::opts::OptionOpts::builder()
            .scope(api::opts::OptionScope::Local)
            .build();

        api::set_option_value("laststatus", 3, &opts).unwrap();

        let statusline = format!(
            "%#Normal# {} Console   {}",
            self.default_server.label(),
            deploy_list
        );

        api::set_option_value("statusline", statusline, &opts).unwrap();

        Ok(())
    }

    pub fn run(&self, dir: &str) -> Result {
        util::print_info(format!("Building project with {}...", self.build_tool));
        api::command("redraw").unwrap();

        self.build_tool.build(dir, &self.java_home)?;

        self.deploy(dir)?;
        self.up()?;

        Ok(())
    }

    pub fn clean(&self) {
        let (deleted, deploy_path) = match self.default_server {
            Server::Jboss => {
                let jboss = self.jboss.as_ref().expect("JBoss not set");
                (jboss.delete_files(), jboss.deploy_path())
            }
            Server::Tomcat => {
                let tomcat = self.tomcat.as_ref().expect("Tomcat not set");
                (tomcat.delete_files(), tomcat.deploy_path())
            }
        };

        match deleted {
            Ok(_) => util::print_info(format!("The content of {deploy_path} has been deleted.")),
            Err(e) => util::print_error(e),
        }
    }

    pub fn deploy(&self, from: &str) -> Result {
        let from = &format!("{}/{}", from, self.build_tool.target_folder());

        let deploy = match self.default_server {
            Server::Jboss => {
                let jboss = self
                    .jboss
                    .as_ref()
                    .ok_or(Error::Msg("JBoss is not set".to_string()))?;
                jboss.deploy(from)
            }
            Server::Tomcat => {
                let tomcat = self
                    .tomcat
                    .as_ref()
                    .ok_or(Error::Msg("Tomcat is not set".to_string()))?;
                tomcat.deploy(from)
            }
        };

        deploy.map_err(Error::Io)
    }

    pub fn get_default_server_as_str(&self) -> &str {
        match self.default_server {
            Server::Jboss => "JBoss",
            Server::Tomcat => "Tomcat",
        }
    }

    pub fn get_tomcat_info(&self) -> Option<(String, String)> {
        let path = self.tomcat.as_ref()?.path.clone();
        let deploys = self
            .tomcat
            .as_ref()?
            .deployed_as_str()
            .unwrap_or("No Data (Tomcat path not set)".to_string());

        Some((path, deploys))
    }

    pub fn get_jboss_info(&self) -> Option<(String, String)> {
        let path = self.jboss.as_ref()?.path.clone();
        let deploys = self
            .jboss
            .as_ref()?
            .deployed_as_str()
            .unwrap_or("No Data (JBoss path not set)".to_string());

        Some((path, deploys))
    }

    fn set_java_home(&self) -> String {
        match self.java_home {
            Some(ref java_home) => format!("export JAVA_HOME={java_home} &&"),
            None => String::new(),
        }
    }

    pub fn get_build_tool(&self) -> &str {
        match self.build_tool {
            BuildTool::Maven => " Maven",
            BuildTool::Gradle => " Gradle",
        }
    }
}

#[derive(Debug)]
pub struct WildcatBuilder(Wildcat);

impl WildcatBuilder {
    pub fn new() -> Self {
        Self(Wildcat {
            console_size: 15,
            java_home: None,
            build_tool: BuildTool::default(),
            default_server: Server::default(),
            jboss: Some(Jboss::default()),
            tomcat: Some(Tomcat::default()),
        })
    }

    pub fn console_size(&mut self, console_size: usize) -> &mut Self {
        self.0.console_size = console_size;
        self
    }

    pub fn build_tool(&mut self, build_tool: BuildTool) -> &mut Self {
        self.0.build_tool = build_tool;
        self
    }

    pub fn default_server(&mut self, default_server: Server) -> &mut Self {
        self.0.default_server = default_server;
        self
    }

    pub fn jboss(&mut self, jboss: Jboss) -> &mut Self {
        self.0.jboss = Some(jboss);
        self
    }

    pub fn tomcat(&mut self, tomcat: Tomcat) -> &mut Self {
        self.0.tomcat = Some(tomcat);
        self
    }

    pub fn java_home(&mut self, java_home: String) -> &mut Self {
        self.0.java_home = Some(java_home);
        self
    }

    pub fn build(self) -> Wildcat {
        self.0
    }
}

#[derive(Debug, Default)]
pub enum BuildTool {
    #[default]
    Maven,
    Gradle,
}

impl BuildTool {
    pub fn build(&self, dir: &str, java_home: &Option<String>) -> Result {
        let (command, opts) = match self {
            BuildTool::Maven => ("mvn", ["-q", "clean", "package"]),
            BuildTool::Gradle => ("gradle", ["-q", "clean", "build"]),
        };

        let mut command = Command::new(command);
        command
            .args(opts)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(dir);

        if let Some(java_home) = java_home {
            command.env("JAVA_HOME", java_home);
        }

        let mut child = command.spawn().map_err(Error::Io)?;

        let mut stdout = child.stdout.take().unwrap();
        let mut stdout_buffer = String::new();
        stdout
            .read_to_string(&mut stdout_buffer)
            .map_err(Error::Io)?;

        let mut stderr = child.stderr.take().unwrap();
        let mut stderr_buffer = String::new();
        stderr
            .read_to_string(&mut stderr_buffer)
            .map_err(Error::Io)?;

        let status = child.wait().map_err(Error::Io)?;

        if !status.success() {
            if stderr_buffer.is_empty() {
                return Err(Error::BuildTool(stdout_buffer));
            }
            return Err(Error::BuildTool(stderr_buffer));
        }

        Ok(())
    }

    pub fn target_folder(&self) -> String {
        match self {
            BuildTool::Maven => "/target".to_string(),
            BuildTool::Gradle => "/build/libs".to_string(),
        }
    }
}

impl Display for BuildTool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildTool::Maven => write!(f, "Maven"),
            BuildTool::Gradle => write!(f, "Gradle"),
        }
    }
}

impl From<String> for BuildTool {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "gradle" => BuildTool::Gradle,
            _ => BuildTool::Maven,
        }
    }
}
