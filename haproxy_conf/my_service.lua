local function my_service(applet)
    local response = "test"

    applet:set_status(200)
    applet:add_header("content-length", string.len(response))
    applet:add_header("content-type", "text/plain")
    applet:start_response()
    applet:send(response)
end

core.register_service("my_service", "http", my_service)
