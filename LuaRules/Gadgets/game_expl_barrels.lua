function gadget:GetInfo()
	return {
		name = "Explosive barrels",
		desc = "Barrels.. that.. explode!",
		author = "gajop",
		date = "April 2015",
		license = "GNU GPL, v2 or later",
		layer = 1,
		enabled = true
	}
end

local LOG_SECTION = "barrel"
local LOG_LEVEL = LOG.NOTICE

local explosionRange = 100
local explosionDamage = 500

if (gadgetHandler:IsSyncedCode()) then

function gadget:UnitDestroyed(unitID, unitDefID, unitTeam, attackerID, attackerDefID, attackerTeam)
    -- boom
    if UnitDefs[unitDefID].customParams.barrel then
        Spring.Log(LOG_SECTION, LOG_LEVEL, "Destroyed explosive barrel: " .. tostring(unitID))
        local x, y, z = Spring.GetUnitPosition(unitID)
        Spring.SpawnCEG("explosion", x, y, z, explosionRange, 100)

        -- TODO: maybe y should matter if we'll have units in the air?
        local nearbyUnits = Spring.GetUnitsInCylinder(x, z, explosionRange)
        -- do massive damage to all nearby units
        -- TODO: maybe make this a function of distance?
        for _, nearbyUnitID in pairs(nearbyUnits) do
            Spring.AddUnitDamage(nearbyUnitID, explosionDamage)
        end
    end
end

end