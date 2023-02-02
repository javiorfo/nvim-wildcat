-- #######################################################
-- # Maintainer:  Javier Orfo                            #
-- # URL:         https://github.com/javi-7/nvim-wildcat #
-- #######################################################

local M = {
    const = {
        WILDCAT_SERVER_CONSOLE = "wildcat_server_console",
        tomcat = {
            LABEL = "Tomcat",
            BASH = "/bin/catalina.sh run"
        },
        jboss = {
            LABEL = "JBoss",
            BASH = "/bin/standalone.sh"
        }
    }
}

local logger = require'osfa.logger':new("Modelizer")
M.logger = logger

function M.get_server_icon(server)
    if server == M.const.tomcat.LABEL then
        return " " .. server
    else
        return "󱄛 " .. server
    end
end

return M
