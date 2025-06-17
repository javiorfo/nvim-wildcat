local spinetta = require 'spinetta'

local lua_wildcat_path = debug.getinfo(1).source:match("@?(.*/)")
local wildcat_root_path = lua_wildcat_path:gsub("/lua/wildcat", "")
local wildcat_log_file = vim.fn.stdpath('log') .. "/wildcat.log"

return {
    build = function()
        if vim.fn.executable("cargo") == 0 then
            vim.notify("Cargo (Rust) is required. Install it to use this plugin and then execute manually :WildcatBuild",
                vim.log.levels.WARN)
            return false
        end

        local script = string.format(
            "%sscript/build.sh %s 2> >( while read line; do echo \"[ERROR][$(date '+%%m/%%d/%%Y %%T')]: ${line}\"; done >> %s)",
            wildcat_root_path,
            wildcat_root_path, wildcat_log_file)
        local spinner = spinetta:new {
            main_msg = "󰄛  Wildcat   Building plugin... ",
            speed_ms = 100,
            on_success = function()
                vim.notify("󰄛  Wildcat     Plugin ready to be used!", vim.log.levels.INFO)
            end
        }

        spinner:start(spinetta.job_to_run(script))
    end
}
