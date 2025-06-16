local M = {}

M.SETTINGS = {
    console_size = 15,
    default = "jboss",
    build_tool = "maven",          -- gradle
    jvm = "optional/path",      -- JAVA_HOME
    jboss = {
        path = "optional/path", -- JBOSS_HOME
        app_base = "standalone/deployments"
    },
    tomcat = {
        path = "optional/path", -- CATALINA_HOME
        app_base = "webapps"
    }
}

return M
