if vim.g.wildcat then
    return
end

vim.g.wildcat = 1

vim.api.nvim_create_user_command('WildcatBuild', 'lua require("wildcat.build").build()', {})
vim.api.nvim_create_user_command('WildcatUp', 'lua require("wildcat").up()', {})
vim.api.nvim_create_user_command('WildcatDown', 'lua require("wildcat").down()', {})
vim.api.nvim_create_user_command('WildcatServer', 'lua require("wildcat.popup").show()', {})
vim.api.nvim_create_user_command('WildcatClean', 'lua require("wildcat").clean()', {})

vim.api.nvim_create_user_command('WildcatDeploy', function(opts)
    local args = opts.fargs[1] or "."
    require 'wildcat'.deploy(args)
end, { nargs = "?" })

vim.api.nvim_create_user_command('WildcatRun', function(opts)
    local args = opts.fargs[1] or "."
    require 'wildcat'.run(args)
end, { nargs = "?" })
