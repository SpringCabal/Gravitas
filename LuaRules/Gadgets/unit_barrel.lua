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
local LOG_LEVEL = LOG.DEBUG

if (gadgetHandler:IsSyncedCode()) then

function rand()
    return math.random()
end

function gadget:UnitDestroyed(unitID, unitDefID, unitTeam, attackerID, attackerDefID, attackerTeam)
    if UnitDefs[unitDefID].customParams.barrel then
        Spring.Log(LOG_SECTION, LOG_LEVEL, "Destroyed explosive barrel: " .. tostring(unitID))
        local x, y, z = Spring.GetUnitPosition(unitID)
        local r = 250 -- explosion radius
        
        --boom
        Spring.PlaySoundFile("sounds/explosion.ogg", 20)
        --draw  
        Spring.SpawnCEG("flashnuke", x,y,z, 0,0,0, 0, 0) --spawn CEG, cause no damage
        Script.LuaRules.FlameRaw(x+5*rand(),y,z+5*rand(), 0,0.1,0, 0,0,0, 500)
        Script.LuaRules.FlameRaw(x+5*rand(),y+5,z+5*rand(), 0,0.2,0, 0,0,0, 500)
        Script.LuaRules.FlameRaw(x+2*rand(),y+10,z+2*rand(), 0,0.5,0, 0,0,0, 500)
        --SendToUnsynced("DeadBarrel", x,y,z, r) 
        
        -- damage all nearby units
        local nearbyUnits = Spring.GetUnitsInCylinder(x, z, r)
        for _, nearbyUnitID in pairs(nearbyUnits) do
            Spring.AddUnitDamage(nearbyUnitID, 500)
        end
    end
end


-- UNSYNCED
else
--

local particleList = {}

function gadget:Initialize()
    gadgetHandler:AddSyncAction("DeadBarrel", DeadBarrel)
end

function gadget:Shutdown()
    gadgetHandler:RemoveSyncAction("DeadBarrel")
end

function DeadBarrel(_,x,y,z, r)
    local partpos = "x*delay,y*delay,z*delay|x=0,y=0,z=0"
    -- might add stuff here
end

function gadget:GameFrame(n)
    if (#particleList>0) then
        GG.Lups.AddParticlesArray(particleList)
        particleList = {}
    end
end

end