-- #######################################################
-- # Maintainer:  Javier Orfo                            #
-- # URL:         https://github.com/javio7/nvim-wildcat #
-- #######################################################

if vim.g.wildcat then
    return
end

vim.g.wildcat = 1

vim.api.nvim_create_user_command('WildcatUp', 'lua require("wildcat.core").wildcat_up()', {})
vim.api.nvim_create_user_command('WildcatDown', 'lua require("wildcat.core").wildcat_down()', {})
vim.api.nvim_create_user_command('WildcatEnableTomcat', 'lua require("wildcat.core").wildcat_enable_tomcat()', {})
vim.api.nvim_create_user_command('WildcatEnableJBoss', 'lua require("wildcat.core").wildcat_enable_jboss()', {})
vim.api.nvim_create_user_command('WildcatInfo', 'lua require("wildcat.core").wildcat_info()', {})
vim.api.nvim_create_user_command('WildcatClean', 'lua require("wildcat.core").wildcat_clean()', {})

vim.api.nvim_create_user_command('WildcatDeploy', function(opts)
    require 'wildcat.core'.wildcat_deploy(opts.fargs)
end, { nargs = "?" })

vim.api.nvim_create_user_command('WildcatRun', function(opts)
    require 'wildcat.core'.wildcat_run(opts.fargs)
end, { nargs = "?" })
