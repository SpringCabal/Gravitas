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
        px = 484.81878662109,
        py = 144.259765625,
        pz = 3279.1237792969,
        mode = 1,
        flipped = -1,
        dy = -0.84741073846817,
        dz = -0.53217732906342,
        fov = 45,
        height = 657.36724853516,
        angle = 0.57099866867065,
        dx = 0,
        name = "ta",
    }
    Spring.SetCameraState(s, 0)
end

function widget:Shutdown()
end

function widget:MouseWheel(up,value)
    -- uncomment this to disable zoom/panning
    return true
end