local popcorn        = require 'popcorn'
local borders        = require 'popcorn.borders'
local wildcat        = require 'wildcat'
local M              = {}

local checked_icon   = " 󰐾  "
local unchecked_icon = "    "

local function create_content()
    local default = wildcat.get_default_server()
    local tomcat_path, tomcat_deploys = wildcat.get_tomcat_info()
    local jboss_path, jboss_deploys = wildcat.get_jboss_info()
    local content = {}

    table.insert(content, { "Select server", "Type" })
    if default == "Tomcat" then
        table.insert(content, { unchecked_icon .. "JBoss" })
        table.insert(content, { checked_icon .. "Tomcat" })
    else
        table.insert(content, { checked_icon .. "JBoss" })
        table.insert(content, { unchecked_icon .. "Tomcat" })
    end

    table.insert(content, { "" })
    table.insert(content, { "Info", "Type" })
    table.insert(content, { "󱄛  JBoss", "Boolean" })

    if jboss_path == "" then
        table.insert(content, { "- Server Not Set" })
    else
        table.insert(content, { "- Path: " .. jboss_path })
        table.insert(content, { "- Deploys: " .. jboss_deploys })
    end

    table.insert(content, { "  Tomcat", "Boolean" })

    if tomcat_path == "" then
        table.insert(content, { "- Server Not Set" })
    else
        table.insert(content, { "- Path: " .. tomcat_path })
        table.insert(content, { "- Deploys: " .. tomcat_deploys })
    end

    return content
end

function M.show()
    local opts = {
        width = 45,
        height = 13,
        title = { "󰄛  Wildcat Servers", "Boolean" },
        footer = { "<CR> to select", "String" },
        border = borders.simple,
        content = create_content(),
        do_after = function()
            vim.api.nvim_win_set_cursor(0, { 2, 0 })
            vim.cmd [[setl noma]]

            vim.api.nvim_buf_set_keymap(0, 'n', '<CR>',
                '<cmd>lua require("wildcat.popup").set()<CR>', { noremap = true, silent = true })
        end
    }

    popcorn:new(opts):pop()
end

function M.set()
    vim.cmd [[setl ma]]
    local line_nr = vim.fn.line('.')

    if line_nr > 1 and line_nr < 4 then
        local selected = vim.fn.getline('.')
        local sel = tostring(selected):gsub(unchecked_icon, checked_icon)
        vim.fn.setline(line_nr, sel)

        local unselect_line = line_nr == 2 and 3 or 2
        local unselected = vim.fn.getline(unselect_line)
        local unsel = tostring(unselected):gsub(checked_icon, unchecked_icon)
        vim.fn.setline(unselect_line, unsel)
    end

    wildcat.switch()

    vim.cmd [[setl noma]]
end

return M
