local function RecursiveReplaceStrings(t, name, replacedMap)
	if (replacedMap[t]) then
		return  -- avoid recursion / repetition
	end
	replacedMap[t] = true
	local changes = {}
	for k, v in pairs(t) do
		if (type(v) == 'string') then
			t[k] = v:gsub("<NAME>", name)
		end
		if (type(v) == 'table') then
			RecursiveReplaceStrings(v, name, replacedMap)
		end
	end 
end

local function ReplaceStrings(t, name)
	local replacedMap = {}
	RecursiveReplaceStrings(t, name, replacedMap)
end

-- The Def:New/Clone/Append helpers from unitdefs_pre.lua are inherited into every
-- def table by value; lowerkeys() then turns them into bogus "new"/"clone"/
-- "append" tags. Strip every function value before the defs reach the engine.
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

-- Process ALL the units!
for name, ud in pairs(UnitDefs) do
	-- Replace all occurences of <NAME> with the respective values
	ReplaceStrings(ud, ud.unitname or name)
	StripFunctions(ud, {})
	if not ud.power then
		local cost = (tonumber(ud.buildcostmetal) or 0) + (tonumber(ud.buildcostenergy) or 0)
		if cost <= 0 then
			ud.power = 1
		end
	end
end
