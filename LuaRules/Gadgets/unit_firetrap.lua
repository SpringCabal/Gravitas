function gadget:GetInfo()
  return {
    name      = "Fire Traps",
    desc      = "I love the small of napalm in the morning",
    author    = "Bluestone",
    date      = "April 2015",
    license   = "GNU GPL, v3",
    layer     = 0,
    enabled   = true,
  }
end


-- FlameRaw format: posx,posy,posz, dirx,diry,dirz, speedx,speedy,speedz, range

-- SYNCED
if gadgetHandler:IsSyncedCode() then

local config = {} -- config[unitID] = radius
local onFireUnits = {} -- onFireUnits[unitID] = {caughtFire=, proximity=}

local fireSpeed = 0.5 -- IF YOU CHANGE THIS THE SKY WILL FALL ON YOUR HEAD
local fireID = UnitDefNames["fire"].id

local currentFrame
local burnTime = 60 -- frames a unit burns for after walking out of a fire trap

function gadget:Initialize()
    -- handle luarules reload
    local currentFrame = Spring.GetGameFrame()
	for _, unitID in pairs(Spring.GetAllUnits()) do
		gadget:UnitCreated(unitID, Spring.GetUnitDefID(unitID), Spring.GetUnitTeam(unitID))
	end    
end

function gadget:GameStart()
    --Spring.SendCommands("cheat")
    --Spring.SendCommands("globallos") 
end

function gadget:UnitCreated(unitID, unitDefID)
    if unitDefID == fireID then
        if Spring.GetUnitRulesParam(unitID, "fireSize") == nil then
            Spring.SetUnitRulesParam(unitID, "fireSize", 500)
        end
        config[unitID] = Spring.GetUnitRulesParam(unitID,"fireSize")
        Spring.SetUnitNoDraw(unitID, true)
        -- Spring.SetUnitNoSelect(unitID, true) --I'm guessing this isn't wanted while we are working
    end
end

function gadget:UnitDestroyed(unitID)
    config[unitID] = nil
end

function RandomUnif(a)
    return a*(2*math.random()-1)
end

function SpawnFire(x,y,z, fireRadius)
    local s = fireSpeed + RandomUnif(0.1)
    local r = fireRadius + RandomUnif(fireRadius/10)
    Script.LuaRules.FlameRaw(x,y,z, 0,s,0, 0,0,0, r)
end

function ProximityInsideFire(unitID, fx,fy,fz)
    -- return the distance inside fire cone, and 0 otherwise
    -- since we have no planes, assume that the fire extends as an infinite cone towards the sky
    if not Spring.ValidUnitID(unitID) then
        return 0
    end
    
    local x,y,z,mx,my,mz = Spring.GetUnitPosition(unitID) --should use midpos, but it seems to be broken for our units
    local r = mx and Spring.GetUnitRadius(unitID) or 0
    
    if (y<fy) then return 0 end
    
    local nx = x - fx
    local ny = y - fy
    local nz = z - fz
    local emitRotSpread = (8 / 360) * (2*math.pi) --from lups_fmale_jitter
    local baseAngle = math.tan(math.atan(emitRotSpread)/fireSpeed) -- angle between the surface of cone and the upwards normal vector
    
    local pDist = math.sqrt(nx*nx+nz*nz) -- perpendicular distance from n to central axis of cone
    local p2Dist = fy * math.tan(baseAngle)
    local p3Dist = p2Dist - pDist -- horizontal distance from (x,y,z) to the surface of the cone
    
    if p3Dist < 0 then return 0 end
    
    local p4Dist = math.cos(baseAngle) * p3Dist -- Euclidean distance from (x,y,z) to the surface of the cone
    return p4Dist    
end

function UpdateFire(n, x,y,z, uID)
    -- draw
    if n%15==0 then
        config[uID] = Spring.GetUnitRulesParam(uID,"fireSize") or 500
        SpawnFire(x,y,z, config[uID])
    end

    -- kill 
    local fireRadius = config[uID]
    local units = Spring.GetUnitsInCylinder(x,z,fireRadius * fireSpeed * 1.1)
    for _,unitID in pairs(units) do
        if not UnitDefs[Spring.GetUnitDefID(unitID)].customParams.invulnerable then
            local p = ProximityInsideFire(unitID, x,y,z)
            if p>0 then
                onFireUnits[unitID] = {caughtFire=currentFrame, proximity=p}
            elseif onFireUnits[unitID] then
                onFireUnits[unitID].proximity = p            
            end
        end
    end
end

function gadget:GameFrame(n)
    currentFrame = n
    for uID,_ in pairs(config) do
        local x,y,z = Spring.GetUnitPosition(uID)
        if x then
            UpdateFire(n, x,y,z, uID)
        end
    end
    
    for unitID,t in pairs(onFireUnits) do
        if t.proximity > 0 then
            local fireSizeMult = 1
            SendToUnsynced("BurnUnitID", unitID, fireSizeMult) 
            Spring.AddUnitDamage(unitID, 2+0.1*t.proximity)                
        elseif (t.caughtFire+burnTime>currentFrame) then
            local fireSizeMult = (t.caughtFire+burnTime - currentFrame) / burnTime
            SendToUnsynced("BurnUnitID", unitID, fireSizeMult) 
            Spring.AddUnitDamage(unitID, 2)        
        else
            onFireUnits[unitID] = nil
        end
    end
    
end

function gadget:UnitPreDamaged(unitID, unitDefID, unitTeam, damage, paralyzer, weaponDefID, projectileID, attackerID, attackerDefID, attackerTeam)
    if unitDefID==fireID then return 0,0 end
    return damage,1.0
end

function gadget:UnitDestroyed(unitID)
    if onFireUnits[unitID] then
        SendToUnsynced("SmokePuffUnitID", unitID, 1.0) 
        SendToUnsynced("BurnUnitID", unitID, 1.0)     
        SendToUnsynced("BurnUnitID", unitID, 1.0)     
    end
    onFireUnits[unitID] = nil
end







-- UNSYNCED
else

function gadget:Initialize()
    gadgetHandler:AddSyncAction("BurnUnitID", BurnUnitID)
    gadgetHandler:AddSyncAction("SmokePuffUnitID", SmokePuffUnitID)
end

function gadget:Shutdown()
    gadgetHandler:RemoveSyncAction("BurnUnitID")
    gadgetHandler:RemoveSyncAction("SmokePuffUnitID")
end
                
function BurnUnitID(_,unitID,fireSizeMult)
    Script.LuaUI.onFireUnit(unitID, fireSizeMult)
end

function SmokePuffUnitID(_,unitID,sizeMult)
    Script.LuaUI(unitID, sizeMult)
end

end