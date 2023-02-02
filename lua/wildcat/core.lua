-- #######################################################
-- # Maintainer:  Javier Orfo                            #
-- # URL:         https://github.com/javi-7/nvim-wildcat #
-- #######################################################

local servers = require'wildcat'.DEFAULT_OPTIONS
local utils = require'wildcat.utils'
local Logger = utils.logger
local unpack = require'osfa.table'.unpack
local read_from = require'osfa.files'.read_from

local jboss = servers.jboss
local tomcat = servers.tomcat
jboss.home = os.getenv(jboss.home) or jboss.home
tomcat.home = os.getenv(tomcat.home) or tomcat.home
local tomcat_deploy_path = tomcat.home .. "/" .. tomcat.app_base
local jboss_deploy_path = jboss.home .. "/" .. jboss.app_base

local M = {}

local function execute(opts)
    local path = opts.home .. opts.sh
    local console = "sp | resize " .. servers.console_size .. " | start | terminal " .. path
    vim.cmd(console)
    vim.cmd("file " .. utils.const.WILDCAT_SERVER_CONSOLE)

    local _, error = pcall(function() require('lualine') end)
    if not error then
        require'lualine'.hide()
    end

    vim.opt_local.laststatus = 3
    local stl = string.format("%%#Normal# %s Console  %s", utils.get_server_icon(opts.label), opts.deploys)
    vim.opt_local.statusline = stl
end

local function tomcat_deploys()
    return io.popen('(ls ' .. tomcat_deploy_path .. ' | grep war$ 2>/dev/null) | tr "\\n" " "')
end

local function jboss_deploys()
    return io.popen('(ls ' .. jboss_deploy_path .. ' | grep ".*\\.\\(ear\\|war\\)$" 2>/dev/null) | tr "\\n" " "')
end

function M.wildcat_up()
    if jboss.default then
        execute {
            home = jboss.home,
            sh = utils.const.jboss.BASH,
            label = utils.const.jboss.LABEL,
            deploys = read_from(jboss_deploys)
        }
    else
        execute {
            home = tomcat.home,
            sh = utils.const.tomcat.BASH,
            label = utils.const.tomcat.LABEL,
            deploys = read_from(tomcat_deploys)
        }
    end
end

function M.wildcat_down()
    pcall(function() vim.cmd('bd! ' .. utils.const.WILDCAT_SERVER_CONSOLE) end)
end

function M.wildcat_enable_tomcat()
    jboss.default = false
    tomcat.default = true
end

function M.wildcat_enable_jboss()
    jboss.default = true
    tomcat.default = false
end

local function get_info_table()
    if jboss.default then
        return {
            "Server  " .. utils.const.jboss.LABEL,
            "Home  " .. jboss.home,
            "App Base  " .. jboss.app_base,
            "Deployed  " .. read_from(jboss_deploys)
        }
    else
        return {
            "Server  " .. utils.const.tomcat.LABEL,
            "Home  " .. tomcat.home,
            "App Base  " .. tomcat.app_base,
            "Deployed  " .. read_from(tomcat_deploys)
        }
    end
end

local function popup()
    local server, home, base, deployed = unpack(get_info_table())
    require 'plenary.popup'.create({ server, "", home, "", base, "", deployed },
        { border = true, pos = "center", title = "Wildcat",
            line = 0, col = 0, minwidth = 20, minheight = 3, time = 10000 })
end

function M.wildcat_info()
    local ok, _ = pcall(popup)
    if not ok then
        local title, home, base, deployed = unpack(get_info_table())
        print(title)
        print(home)
        print(base)
        print(deployed)
    end
end

function M.wildcat_clean()

    local function clean(path, regex)
        regex = regex or "/*.war*"
        os.execute("rm -f " .. path .. regex)
    end

    if jboss.default then
        pcall(function()
            clean(jboss_deploy_path)
            clean(jboss_deploy_path, "/*.ear*")
            Logger:info("The content of " .. jboss_deploy_path .. " has been deleted.")
        end)
    else
        pcall(function()
            clean(tomcat_deploy_path)
            Logger:info("The content of " .. tomcat_deploy_path .. " has been deleted.")
        end)
    end
end

function M.wildcat_deploy(args)
    args[1] = args[1] or "."

    local function deploy(values, path, file_ext)
        file_ext = file_ext or "*.war"
        os.execute("if test -f " .. values .. "/target/" .. file_ext .. "; then cp " .. values .. "/target/" .. file_ext .. " " .. path .. "; fi")
    end

    if jboss.default then
        deploy(args[1], jboss_deploy_path)
        deploy(args[1], jboss_deploy_path, "*.ear")
    else
        deploy(args[1], tomcat_deploy_path)
    end
    Logger:info("Done!")
end

function M.wildcat_run(args)
    if vim.fn.executable("mvn") ~= 0 then
        args[1] = args[1] or "."
        Logger:info("Building...")
        local command = "cd " .. args[1] .. "/ && mvn -q clean package &> /dev/null; echo $?"
        local ok = vim.fn.system(command)

        if tonumber(ok) == 0 then
            M.wildcat_deploy(args)
            vim.cmd("redraw")
            M.wildcat_up()
        else
            vim.cmd("redraw")
            Logger:error("Error executing 'mvn clean package'. Run Maven manually to get more info.")
        end
    else
        Logger:error("Maven is not installed. It's required to execute WildcatRun.")
    end
end

return M
