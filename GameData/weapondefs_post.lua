--------------------------------------------------------------------------------
-- This file shadows the engine's gamedata/weapondefs_post.lua, which still has
-- to run: it hoists unit-embedded weaponDefs into WeaponDefs and converts the
-- pre-0.83 tags. So chain to it first, then apply our own fixups.
--------------------------------------------------------------------------------

if VFS.FileExists("gamedata/weapondefs_post.lua", VFS.BASE) then
	VFS.Include("gamedata/weapondefs_post.lua", nil, VFS.BASE)
end

--------------------------------------------------------------------------------
-- The Def:New/Clone/Append helpers from unitdefs_pre.lua are inherited into
-- every def table by value. lowerkeys() then turns them into "new"/"clone"/
-- "append" tags, which the engine reports as unknown. Strip every function
-- value before the defs reach the engine, along with other invalid tags.
--------------------------------------------------------------------------------

local function StripFunctions(t, seen)
	if seen[t] then
		return
	end
	seen[t] = true
	for k, v in pairs(t) do
		if type(v) == "function" then
			t[k] = nil
		elseif type(v) == "table" then
			StripFunctions(v, seen)
		end
	end
end

for _, wd in pairs(WeaponDefs) do
	StripFunctions(wd, {})
	wd.maxdamage = nil
	wd.onlytargetcategory = nil
	wd.startsmoke = nil
	wd.endsmoke = nil
	if wd.explosiongenerator == "custom:NONE" or wd.explosiongenerator == "custom:nothing" then
		wd.explosiongenerator = ""
	end
	if wd.cegtag == "missiletrailred" then
		wd.cegtag = ""
	end
end
