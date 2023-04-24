# nvim-wildcat
### The Wildcat (WILDfly and tomCAT)
*nvim-wildcat is a Neovim plugin written in Lua for running Wildfly, JBoss EAP or Tomcat servers. A useful plugin for Java (or other JVM language) developers who still use the aforemention servers. The base idea was to have a plugin like Eclipse server plugin*

## Caveats
- Ensure you have `Java` installed. Although it is a requisite to have Java to run Wildfly, JBoss EAP or Tomcat; this is the first requisite. 
- Ensure you have Maven installed. nvim-wildcat will throw an error by executing WildcatRun if `Maven` is not installed.
- This plugin has been developed on and for Linux following open source philosophy.

## Installation
`Packer`
```lua
use {
    'charkuils/nvim-wildcat',
    requires = 'charkuils/nvim-popcorn'
}
```
`Lazy`
```lua
{
    'charkuils/nvim-wildcat', lazy = true,
    dependencies = { 'charkuils/nvim-popcorn' }
}
```

## Settings
### Default Settings
#### The following are the basics settings you need to use nvim-wildcat

- By default these are de settings. You can modify one or multiple values in your _init.lua_ or _init.vim_:
```lua
require'wildcat'.setup{
   console_size = 15,
   jboss = {
       home = "JBOSS_HOME",
       app_base = "standalone/deployments",
       default = true
   },
   tomcat = {
       home = "CATALINA_HOME",
       app_base = "webapps",
       default = false
   }
}
```

- nvim-wildcat will take the 'home' values as environment variables and if they not exist, then will take the values as absolute paths.

#### Example of custom settings:
- If wanted to set only Tomcat server as default and set an absolute path, just set it this way:
```lua
require'wildcat'.setup{
    tomcat = {
        home = "/path/to/tomcat",
        default = true
    }
}
```

## Usage
### To deploy on the server
- This command will run `mvn -q clean package` to build your app, it will deploy the war/ear in the deployments folder and it will start the server. To run it, inside the app root folder execute this command `:WildcatRun`
- If you want to run this command outside the app root folder, pass the path by parameter `:WildcatRun /path/to/your/app/root/folder`

### List of commands:
| Command | Description                       |
| -------------- | --------------------------------- |
| `:WildcatClean`  | This command will delete the deployed files in _app base_ folder of the server |
| `:WildcatDeploy` | This command will copy the current or absolute path of a war/ear the to _app base_ folder of the server |
| `:WildcatDown`   | This command will stop the server |
| `:WildcatEnableJBoss` | This command will enable JBoss (is the default) |
| `:WildcatEnableTomcat` | This command will enable Tomcat |
| `:WildcatInfo` | This command will show info about your config (server, home, app base and deployed files) |
| `:WildcatRun`    | This command will build with Maven, copy the war/ear file to the server and start the server|
| `:WildcatUp` | This command will start the server |

## Screenshots

<img src="https://github.com/charkuils/img/blob/master/nvim-wildcat/wildcat2.gif?raw=true" alt="wildcat" style="width:1000px;"/>

**NOTE:** The colorscheme **malt** from [nvim-whisky](https://github.com/charkuils/nvim-whisky) is used in this image

## Documentation
- nvim-wilcat comes with built-in doc `:help wildcat`

## Troubleshooting
- When running Tomcat sometimes an execution permission is needed for catalina.sh. This will help:
```console
[user@host ~]$ chmod +x $CATALINA_HOME/bin/catalina.sh
```

## Support Charkuils' Work

<img src="https://github.com/charkuils/img/blob/master/binance/BinancePayQR.png?raw=true" alt="binance" style="width:300px;"/>
