use std::{fs, io};

use nvim_oxi::conversion::FromObject;

#[derive(Debug, Default)]
pub enum Server {
    #[default]
    Jboss,
    Tomcat,
}

impl Server {
    pub fn label(&self) -> &'static str {
        match self {
            Server::Jboss => "󱄛  JBoss",
            Server::Tomcat => "  Tomcat",
        }
    }
}

impl From<String> for Server {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "tomcat" => Server::Tomcat,
            _ => Server::Jboss,
        }
    }
}

#[derive(Debug)]
pub struct Jboss {
    pub path: String,
    app_base: String,
}

impl Jboss {
    pub fn run_path(&self) -> String {
        format!("{}/bin/standalone.sh", self.path)
    }

    pub fn deploy_path(&self) -> String {
        format!("{}/{}", self.path, self.app_base)
    }

    pub fn deployed_as_str(&self) -> io::Result<String> {
        let entries = fs::read_dir(self.deploy_path())?;

        let war_files: Vec<String> = entries
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "war" || ext == "ear" {
                            return path.file_name()?.to_str().map(|s| s.to_owned());
                        }
                    }
                }
                None
            })
            .collect();

        if war_files.is_empty() {
            Ok("No deployments".to_string())
        } else {
            Ok(war_files.join(", "))
        }
    }

    pub fn delete_files(&self) -> io::Result<()> {
        for entry_result in fs::read_dir(self.deploy_path())? {
            let entry = entry_result?;
            let path = entry.path();
            if path.is_file() {
                fs::remove_file(path)?;
            } else if path.is_dir() {
                fs::remove_dir_all(path)?;
            }
        }
        Ok(())
    }

    pub fn deploy(&self, from: &str) -> io::Result<()> {
        for entry in fs::read_dir(from)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "war" || ext == "ear" {
                        let file_name = path.file_name().unwrap().to_str().unwrap();
                        let dest_path = format!("{}/{}", self.deploy_path(), file_name);
                        fs::copy(&path, dest_path)?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl ServerParser for Jboss {
    const ENV_VAR: &'static str = "JBOSS_HOME";
    const APP_BASE: &'static str = "standalone/deployments";
    const SERVER_LUA_PROP: &'static str = "jboss.path";
}

impl FromObject for Jboss {
    fn from_object(object: nvim_oxi::Object) -> Result<Self, nvim_oxi::conversion::Error> {
        let (path_str, app_base) = <Jboss as ServerParser>::from_object(object)?;

        Ok(Self {
            path: path_str,
            app_base,
        })
    }
}

#[derive(Debug)]
pub struct Tomcat {
    pub path: String,
    app_base: String,
}

impl Tomcat {
    pub fn run_path(&self) -> String {
        format!("{}/bin/catalina.sh run", self.path)
    }

    pub fn deploy_path(&self) -> String {
        format!("{}/{}", self.path, self.app_base)
    }

    pub fn deployed_as_str(&self) -> io::Result<String> {
        let entries = fs::read_dir(self.deploy_path())?;

        let war_files: Vec<String> = entries
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "war" {
                            return path.file_name()?.to_str().map(|s| s.to_owned());
                        }
                    }
                }
                None
            })
            .collect();

        if war_files.is_empty() {
            Ok("No deployments".to_string())
        } else {
            Ok(war_files.join(", "))
        }
    }

    pub fn delete_files(&self) -> io::Result<()> {
        for entry_result in fs::read_dir(self.deploy_path())? {
            let entry = entry_result?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if dir_name == "docs"
                        || dir_name == "ROOT"
                        || dir_name == "examples"
                        || dir_name == "host-manager"
                        || dir_name == "manager"
                    {
                        continue;
                    }
                }
            } else if path.is_file() {
                fs::remove_file(path)?;
            }
        }
        Ok(())
    }

    pub fn deploy(&self, from: &str) -> io::Result<()> {
        for entry in fs::read_dir(from)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "war" {
                        let file_name = path.file_name().unwrap().to_str().unwrap();
                        let dest_path = format!("{}/{}", self.deploy_path(), file_name);
                        fs::copy(&path, dest_path)?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl ServerParser for Tomcat {
    const ENV_VAR: &'static str = "CATALINA_HOME";
    const APP_BASE: &'static str = "webapps";
    const SERVER_LUA_PROP: &'static str = "tomcat.path";
}

impl FromObject for Tomcat {
    fn from_object(object: nvim_oxi::Object) -> Result<Self, nvim_oxi::conversion::Error> {
        let (path_str, app_base) = <Tomcat as ServerParser>::from_object(object)?;

        Ok(Self {
            path: path_str,
            app_base,
        })
    }
}

trait ServerParser {
    const ENV_VAR: &'static str;
    const APP_BASE: &'static str;
    const SERVER_LUA_PROP: &'static str;

    fn from_object(
        object: nvim_oxi::Object,
    ) -> Result<(String, String), nvim_oxi::conversion::Error> {
        let dict = nvim_oxi::Dictionary::from_object(object)?;

        let path_str = match dict
            .get("path")
            .and_then(|obj| String::from_object(obj.clone()).ok())
        {
            Some(path) => path,
            None => std::env::var(Self::ENV_VAR).map_err(|_| {
                nvim_oxi::conversion::Error::Other(format!(
                    "Environment variable {} or {} in wildcat setup is required",
                    Self::ENV_VAR,
                    Self::SERVER_LUA_PROP
                ))
            })?,
        };

        let app_base = dict
            .get("app_base")
            .and_then(|obj| String::from_object(obj.clone()).ok())
            .unwrap_or(Self::APP_BASE.to_string());

        Ok((path_str, app_base))
    }
}
