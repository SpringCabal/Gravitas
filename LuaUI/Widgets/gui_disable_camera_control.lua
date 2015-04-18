--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

function widget:GetInfo()
	return {
		name      = "Disable camera control",
		desc      = "Disables camera zooming and panning",
		author    = "gajop",
		date      = "WIP",
		license   = "GPLv2",
		version   = "0.1",
		layer     = -1000,
		enabled   = true,  --  loaded by default?
		handler   = true,
		api       = true,
		hidden    = true,
	}
end

function widget:Initialize()
    for k, v in pairs(Spring.GetCameraState()) do
        print(k .. " = " .. tostring(v) .. ",")
    end
    local devMode = (tonumber(Spring.GetModOptions().play_mode) or 0) == 0
    if devMode then
        widgetHandler:RemoveWidget(widget)
        return
    end
--     s = {
--         dist = 450,
--         px = 1998.0675048828,
--         py = 91.71875,
--         pz = 3952.1501464844,
--         rz = 0,
--         dx = -0,
--         dy = -0.7716411948204,
--         dz = -0.63605803251266,
--         fov = 45,
--         ry = -0.37921515107155,
--         mode = 2,
--         rx = 2.4522137641907,
--         name = spring,
--     }
    s = {
        px = 1989.8743896484,
        py = 91.853881835938,
        pz = 3860.1606445312,
        zscale = 0.6866455078125,
        flipped = -1,
        dy = -0.82436865568161,
        name = "sm",
        fov = 45,
        height = 620.48107910156,
        mode = 5,
        dx = 0,
        dz = -0.56604903936386,
    }
    Spring.SetCameraState(s, 0)
end

function widget:Shutdown()
end

function widget:MouseWheel(up,value)
    -- uncomment this to disable zoom/panning
    return true
end