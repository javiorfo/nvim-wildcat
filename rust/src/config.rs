use std::path::PathBuf;

use nvim_oxi::{Dictionary, conversion::FromObject};

#[derive(Debug, Default)]
pub struct Wildcat {
    console_size: usize,
    jvm: String,
    build_tool: BuildTool,
    default_server: Server,
    jboss: Option<Jboss>,
    tomcat: Option<Tomcat>,
}

impl Wildcat {
    pub fn switch(&mut self) {
        match self.default_server {
            Server::Jboss => self.default_server = Server::Tomcat,
            Server::Tomcat => self.default_server = Server::Jboss,
        }
    }
}

#[derive(Debug)]
pub struct WildcatBuilder(Wildcat);

impl WildcatBuilder {
    pub fn new(jvm: String) -> Self {
        Self(Wildcat {
            console_size: 15,
            jvm,
            build_tool: BuildTool::default(),
            default_server: Server::default(),
            jboss: None,
            tomcat: None,
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

impl From<String> for BuildTool {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "maven" => BuildTool::Maven,
            "gradle" => BuildTool::Gradle,
            _ => BuildTool::Maven,
        }
    }
}

#[derive(Debug, Default)]
pub enum Server {
    #[default]
    Jboss,
    Tomcat,
}

impl From<String> for Server {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "jboss" => Server::Jboss,
            "tomcat" => Server::Tomcat,
            _ => Server::Jboss,
        }
    }
}

#[derive(Debug)]
pub struct Jboss {
    path: PathBuf,
    app_base: String,
}

impl FromObject for Jboss {
    fn from_object(object: nvim_oxi::Object) -> Result<Self, nvim_oxi::conversion::Error> {
        let dict = Dictionary::from_object(object)?;

        let path_str = match dict
            .get("path")
            .and_then(|obj| String::from_object(obj.clone()).ok())
        {
            Some(path) => path,
            None => std::env::var("JBOSS_HOME").map_err(|_| {
                nvim_oxi::conversion::Error::Other(
                    "Environment variable JBOSS_HOME or jboss.path in wildcat setup is required"
                        .to_string(),
                )
            })?,
        };

        let app_base = dict
            .get("app_base")
            .and_then(|obj| String::from_object(obj.clone()).ok())
            .unwrap_or("standalone/deployments".to_string());

        Ok(Self {
            path: std::path::PathBuf::from(path_str),
            app_base,
        })
    }
}

#[derive(Debug)]
pub struct Tomcat {
    path: PathBuf,
    app_base: String,
}

impl FromObject for Tomcat {
    fn from_object(object: nvim_oxi::Object) -> Result<Self, nvim_oxi::conversion::Error> {
        let dict = Dictionary::from_object(object)?;

        let path_str = match dict
            .get("path")
            .and_then(|obj| String::from_object(obj.clone()).ok())
        {
            Some(path) => path,
            None => std::env::var("CATALINA_HOME").map_err(|_| {
                nvim_oxi::conversion::Error::Other(
                    "Environment variable CATALINA_HOME or tomcat.path in wildcat setup is required".to_string(),
                )
            })?,
        };

        let app_base = dict
            .get("app_base")
            .and_then(|obj| String::from_object(obj.clone()).ok())
            .unwrap_or("webapps".to_string());

        Ok(Self {
            path: std::path::PathBuf::from(path_str),
            app_base,
        })
    }
}
