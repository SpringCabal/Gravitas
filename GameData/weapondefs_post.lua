-- The engine's own gamedata/weapondefs_post.lua still has to run (it converts
-- old tags and moves weapons defined inside unit defs into WeaponDefs), and a
-- game file of the same name replaces it, so run it first.
VFS.Include("gamedata/weapondefs_post.lua", nil, VFS.BASE)

-- The Def:New/Clone/Append helpers from unitdefs_pre.lua are copied into every
-- def, and the engine warns about each as an unknown "new", "clone" and
-- "append" tag. Drop them.
for _, wd in pairs(WeaponDefs) do
	for key, value in pairs(wd) do
		if type(value) == "function" then
			wd[key] = nil
		end
	end
end
