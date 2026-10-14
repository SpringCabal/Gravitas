-- Explicitly empty explosion generators: the weapons that name custom:NONE
-- (the gravity beams) and custom:nothing (the NOWEAPON placeholder in
-- GameData/weapondefs_pre.lua) must not draw an explosion.
return {
	["NONE"] = {
		usedefaultexplosions = false,
	},
	["nothing"] = {
		usedefaultexplosions = false,
	},
}
