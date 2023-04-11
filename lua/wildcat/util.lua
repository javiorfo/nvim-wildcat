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

local logger = require'wildcat.logger':new("Wildcat")
M.Logger = logger

function M.table_length(table)
    local count = 0
    for _ in pairs(table) do count = count + 1 end
    return count
end

function M.unpack(tab, start, stop)
    if not start then start = 1 end
    if not stop then stop = #tab end
    if start == stop then
        return tab[start]
    else
        return tab[start], unpack(tab, start + 1, stop)
    end
end

function M.read_from(fn, message)
    message = message or "Empty"
    local handle = fn()

    if handle then
        local result = handle:read("*a")
        handle:close()
        if #result == 0 then return message else return result end
    end
end

function M.get_server_icon(server)
    if server == M.const.tomcat.LABEL then
        return " " .. server
    else
        return "󱄛 " .. server
    end
end

return M
