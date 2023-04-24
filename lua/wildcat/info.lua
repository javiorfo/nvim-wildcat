local popcorn = require'popcorn'
local borders = require'popcorn.borders'
local M = {}

function M.show(text)
    local opts = {
        width = 49,
        height = 11,
        title = { "WILDCAT", "Boolean" },
        border = borders.double_border,
        content = text
    }

    popcorn:new(opts):pop()
end

return M
