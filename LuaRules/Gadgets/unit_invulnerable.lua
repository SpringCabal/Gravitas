function gadget:GetInfo()
	return {
		name = "Invulnerability & Nonparalyzability",
		desc = "Blocks damage to invulnerable/nonparalyzable units",
		author = "Robert Paulson",
		date = "",
		license = "Rabbits",
		layer = 100,
		enabled = true
	}
end

if (gadgetHandler:IsSyncedCode()) then

function gadget:UnitCreated(unitID, unitDefID, unitTeam, builderID)
    if tonumber(UnitDefs[unitDefID].customParams.invulnerable) == 1 then
        Spring.SetUnitRulesParam(unitID, "invulnerable", 1)
    end

    if tonumber(UnitDefs[unitDefID].customParams.nonparalyzable) == 1 then
        Spring.SetUnitRulesParam(unitID, "nonparalyzable", 1)
    end    
end

function gadget:UnitPreDamaged(unitID, unitDefID, unitTeam, damage, paralyzer, weaponDefID, projectileID, attackerID, attackerDefID, attackerTeam)
    if Spring.GetUnitRulesParam(unitID, "invulnerable") == 1 and not paralyzer then
        return 0,0
    end
    if Spring.GetUnitRulesParam(unitID, "nonparalyzable") ==1 and paralyzer then
        return 0,0
    end
    return damage,1.0
end

else
return false
end