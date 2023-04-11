local M = {}
function M.show(text)
        local buf_border = vim.api.nvim_create_buf(false, true)
        local ui = vim.api.nvim_list_uis()[1]
        local width = 49
        local height = 6

        local lines = {
            "┌─────────────────── WILDCAT ──────────────────┐",
            "│                                              │",
            "│                                              │",
            "│                                              │",
            "│                                              │",
            "└──────────────────────────────────────────────┘",
        }
        vim.api.nvim_buf_set_lines(buf_border, 0, -1, true, lines)

        local opts_border = { relative = 'editor',
            width = width,
            height = height,
            col = (ui.width / 2) - (width / 2),
            row = (ui.height / 2) - (height / 2),
            style = 'minimal',
            focusable = false
        }

        vim.api.nvim_open_win(buf_border, true, opts_border)
        vim.cmd("syn keyword wildcatInfoTitle WILDCAT | hi link wildcatInfoTitle Boolean")

        local opts_text = {
            relative = 'editor',
            row = opts_border.row + 1,
            height = opts_border.height - 2,
            col = opts_border.col + 2,
            width = opts_border.width - 4,
            style = 'minimal',
        }

        local buf_text = vim.api.nvim_create_buf(false, true)
        vim.api.nvim_open_win(buf_text, true, opts_text)
        vim.cmd("syn keyword wildcatInfoText Server App Base Deployed Home | hi link wildcatInfoText Boolean")

        vim.api.nvim_buf_set_lines(buf_text, 0, -1, true, text)

        vim.cmd(string.format("au BufLeave <buffer> bd %d | quit", buf_border))
        vim.cmd("nnoremap <buffer> <esc> <cmd>quit<cr>")
end

return M
