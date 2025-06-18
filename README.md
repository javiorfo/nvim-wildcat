# nvim-wildcat
### The Wildcat (WILDfly and tomCAT)
*nvim-wildcat is a Neovim plugin written in Lua for running Wildfly, JBoss EAP or Tomcat servers. A useful plugin for Java (or other JVM language) developers who still use the aforemention servers. The base idea was to have a plugin like Eclipse server plugin*

## Caveats
- This plugin requires `Rust (Cargo)` to be installed
- Ensure you have `Java` installed. Although it is a requisite to have Java to run `Wildfly`, `JBoss EAP` or `Tomcat`; this is the first requisite. 
- Ensure you have `Maven` or `Gradle` installed. nvim-wildcat will throw an error by executing WildcatRun if one of these is not installed.
- This plugin has been developed on and for Linux following open source philosophy.

## Installation
`Lazy`
```lua
{
    'javiorfo/nvim-wildcat',
    lazy = true,
    cmd = { "WildcatBuild", "WildcatRun", "WildcatUp", "WildcatServer" },
    dependencies = { 'javiorfo/nvim-popcorn', 'javiorfo/nvim-spinetta' },
    build = function()
        require 'wildcat.build'.build()
    end,
    opts = {

        -- Optional. Default 15
        -- The size of the server console
        console_size = 15,

        -- Optional. Default "jboss"
        -- Default server (jboss or tomcat)
        default_server = "jboss",

        -- Optional. Default "maven"
        -- Build tool (maven or gradle)
        build_tool = "maven",

        -- Optional. Default JAVA_HOME from the system
        -- If a different java home is required
        java_home = "/path/to/openjdk",

        -- Optional 
        -- Default path JBOSS_HOME from the system
        -- Default app_base "standalone/deployments"
        jboss = {
            path = "/path/to/jboss",
            app_base = "standalone/deployments",
        },

        -- Optional 
        -- Default path CATALINA_HOME from the system
        -- Default app_base "webapps"
        tomcat = {
            path = "/path/to/tomcat",
            app_base = "webapps",
        }
    }
}
```

#### Example of custom settings:
- If wanted to set only Tomcat server as default and set an absolute path, just set it this way:
```lua
require'wildcat'.setup{
    build_tool = "gradle",
    default_server = "tomcat",
    tomcat = {
        path = "/path/to/tomcat",
    }
}
```

## Usage
### To deploy on the server
- This command will run Maven or Gradle and will deploy the war/ear in the deployments folder and it will start the server. To run it, inside the app root folder execute this command `:WildcatRun`
- If you want to run this command outside the app root folder, pass the path by parameter `:WildcatRun /path/to/your/app/folder`

### List of commands:
| Command | Description                       |
| -------------- | --------------------------------- |
| `:WildcatBuild`  | This command will build Rust binary |
| `:WildcatClean`  | This command will delete the deployed files in _app base_ folder of the server |
| `:WildcatDeploy` | This command will copy the current or absolute path of a war/ear the to _app base_ folder of the server (taken from */target* (Maven) or */build/libs* (Gradle) |
| `:WildcatDown`   | This command will stop the server |
| `:WildcatServer` | This command will open a popup to select JBoss or Tomcat and will show some servers info |
| `:WildcatRun`    | This command will build with Maven, copy the war/ear file to the server and start the server|
| `:WildcatUp` | This command will start the server |

## Screenshots

<img src="https://github.com/javiorfo/img/blob/master/nvim-wildcat/wildcat2.gif?raw=true" alt="wildcat" />

**NOTE:** The colorscheme **umbra** from [nvim-nyctophilia](https://github.com/javiorfo/nvim-nyctophilia) is used in this image

## Documentation
- nvim-wilcat comes with built-in doc `:help wildcat`

## Troubleshooting
- When running Tomcat sometimes an execution permission is needed for catalina.sh. This will help:
```console
[user@host ~]$ chmod +x $CATALINA_HOME/bin/catalina.sh
```

---

### Donate
- **Bitcoin** [(QR)](https://raw.githubusercontent.com/javiorfo/img/master/crypto/bitcoin.png)  `1GqdJ63RDPE4eJKujHi166FAyigvHu5R7v`
- [Paypal](https://www.paypal.com/donate/?hosted_button_id=FA7SGLSCT2H8G) 
