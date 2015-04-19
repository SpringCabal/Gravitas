function widget:GetInfo()
	return {
		name      = "Pause On Space",
		desc      = "Pauses/unpauses the game when space (meta) is pressed",
		author    = "Bluestone",
		date      = "April 2015",
		license   = "Horses",
		layer     = 0,
		enabled   = true
	}
end

include('keysym.h.lua')
local SPACE = KEYSYMS.SPACE
function widget:KeyPress(key, mods, isRepeat)
    if key==SPACE then
        Spring.SendCommands("pause")
        return true
    end
end

