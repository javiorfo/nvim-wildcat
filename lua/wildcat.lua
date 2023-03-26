-- ##########################################################
-- # Maintainer: System Malt                                #
-- # URL:        https://github.com/systemmalt/nvim-wildcat #
-- ##########################################################

local util = require'wildcat.util'
local Logger = util.Logger

local M = {}

vim.cmd [[
    augroup wildcat_statusline
        autocmd!
        autocmd BufDelete wildcat_server_console lua pcall(function() require('lualine').hide({unhide=true}) end)
    augroup END
]]

M.DEFAULT_OPTIONS = {
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

local function is_server_opts_valid(server, server_name)
    server_name = server_name or util.const.jboss.LABEL
    local home = server.home
    local app_base = server.app_base
    local default = server.default

    if not home and not app_base and not default then
        Logger:warn("Neither 'home', 'app_base' nor 'default' keys are set in " .. server_name .. " in Wildcat setup!")
        return false
    end
    if home then
        if type(home) ~= "string" then
            Logger:warn(server_name .. " 'home' key must be string")
            return false
        end
    end
    if app_base then
        if type(app_base) ~= "string" then
            Logger:warn(server_name .. " 'app_base' key must be string")
            return false
        end
    end
    if default then
        if type(default) ~= "boolean" then
            Logger:warn(server_name .. " 'default' key must be boolean")
            return false
        end
    end
    return true
end

local function is_valid(opts)
    if opts.console_size then
        if type(opts.console_size) ~= "number" then
            Logger:warn("'console_size' key must be number")
            return false
        end
    end
    if opts.jboss then
       return is_server_opts_valid(opts.jboss)
    end
    if opts.tomcat then
       return is_server_opts_valid(opts.tomcat, util.const.tomcat.LABEL)
    end
    return true
end

local function set_opts(opts)
    if opts.jboss then
        M.DEFAULT_OPTIONS.jboss.home = opts.jboss.home or M.DEFAULT_OPTIONS.jboss.home
        M.DEFAULT_OPTIONS.jboss.app_base = opts.jboss.app_base or M.DEFAULT_OPTIONS.jboss.app_base
        M.DEFAULT_OPTIONS.jboss.default = opts.jboss.default or M.DEFAULT_OPTIONS.jboss.default
    end
    if opts.tomcat then
        M.DEFAULT_OPTIONS.tomcat.home = opts.tomcat.home or M.DEFAULT_OPTIONS.tomcat.home
        M.DEFAULT_OPTIONS.tomcat.app_base = opts.tomcat.app_base or M.DEFAULT_OPTIONS.tomcat.app_base
        M.DEFAULT_OPTIONS.tomcat.default = opts.tomcat.default or M.DEFAULT_OPTIONS.tomcat.default

        if M.DEFAULT_OPTIONS.tomcat.default then M.DEFAULT_OPTIONS.jboss.default = false end
    end
    if opts.console_size then
        M.DEFAULT_OPTIONS.console_size = opts.console_size
    end
end

function M.setup(opts)
    if util.table_length(opts) > 0 then
       if is_valid(opts) then
            set_opts(opts)
       end
    end
end

return M
