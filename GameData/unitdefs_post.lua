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

-- Process ALL the units!
for name, ud in pairs(UnitDefs) do
	-- Replace all occurences of <NAME> with the respective values
	ReplaceStrings(ud, ud.unitname or name)
	-- Nothing here has a cost, so the engine's default power (from the
	-- cost) is 0, which it warns about: experience divides by power.
	ud.power = ud.power or 1
end
