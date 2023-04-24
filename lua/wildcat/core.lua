local servers = require'wildcat'.DEFAULT_OPTIONS
local util = require'wildcat.util'
local Logger = require'wildcat.logger':new("Wildcat")
local info = require'wildcat.info'

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
    vim.cmd("file " .. util.const.WILDCAT_SERVER_CONSOLE)

    local _, error = pcall(function() require('lualine') end)
    if not error then
        require'lualine'.hide()
    end

    vim.opt_local.laststatus = 3
    local stl = string.format("%%#Normal# %s Console  %s", util.get_server_icon(opts.label), opts.deploys)
    vim.opt_local.statusline = stl
end

local function tomcat_deploys()
    return io.popen('[[ -d '..  tomcat_deploy_path ..' ]] && (ls ' .. tomcat_deploy_path .. ' | grep war$ 2>/dev/null) | tr "\\n" " "')
end

local function jboss_deploys()
    return io.popen('[[ -d '..  jboss_deploy_path ..' ]] && (ls ' .. jboss_deploy_path .. ' | grep ".*\\.\\(ear\\|war\\)$" 2>/dev/null) | tr "\\n" " "')
end

function M.wildcat_up()
    if jboss.default then
        execute {
            home = jboss.home,
            sh = util.const.jboss.BASH,
            label = util.const.jboss.LABEL,
            deploys = util.read_from(jboss_deploys)
        }
    else
        execute {
            home = tomcat.home,
            sh = util.const.tomcat.BASH,
            label = util.const.tomcat.LABEL,
            deploys = util.read_from(tomcat_deploys)
        }
    end
end

function M.wildcat_down()
    pcall(function() vim.cmd('bd! ' .. util.const.WILDCAT_SERVER_CONSOLE) end)
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
            { "Server  ", "Type" }, { "    " .. util.const.jboss.LABEL },
            { "Home  " , "Type" }, { "    " .. jboss.home },
            { "App Base  " , "Type" }, { "    " .. jboss.app_base },
            { "Deployed  " , "Type" }, { "    " .. util.read_from(jboss_deploys) }
        }
    else
        return {
            { "Server  ", "Type" }, { "    " .. util.const.tomcat.LABEL },
            { "Home  " , "Type" }, { "    " .. tomcat.home },
            { "App Base  " , "Type" }, { "    " .. tomcat.app_base },
            { "Deployed  " , "Type" }, { "    " .. util.read_from(tomcat_deploys) }
        }
    end
end

function M.wildcat_info()
    info.show(get_info_table())
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
