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

local mapX = Game.mapSizeX
local mapZ = Game.mapSizeZ

local config = {} -- table of {x=,y=,z=,r=}

-- FlameRaw format: posx,posy,posz, dirx,diry,dirz, speedx,speedy,speedz, range

-- SYNCED
if gadgetHandler:IsSyncedCode() then

local fireRadius = 375
local fireSpeed = 0.5

local fireID = UnitDefNames["fire"].id

function gadget:Initialize()
    --[[
    -- testing
    local x = mapX/2
    local z = mapZ/2
    local y = Spring.GetGroundHeight(x,z)
    config[1] = {x=x,y=y,z=z}   
    ]]
    
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
        local x,y,z = Spring.GetUnitPosition(unitID)
        local y = Spring.GetGroundHeight(x,z)
        config[unitID] = true
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

function SpawnFire(x,y,z)
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
    local emitRotSpread = (8 / 360) * (2*math.pi)--from lups_fmale_jitter
    local baseAngle = math.tan(math.atan(emitRotSpread)/fireSpeed) -- angle between the surface of cone and the upwards normal vector
    
    local pDist = math.sqrt(nx*nx+nz*nz) -- perpendicular distance from n to central axis of cone
    local p2Dist = fy * math.tan(baseAngle)
    local p3Dist = p2Dist - pDist -- horizontal distance from (x,y,z) to the surface of the cone
    
    if p3Dist < 0 then return 0 end
    
    local p4Dist = math.cos(baseAngle) * p3Dist -- Euclidean distance from (x,y,z) to the surface of the cone
    return p4Dist    
end

function UpdateFire(n,x,y,z)
    if n%15==0 then
        SpawnFire(x,y,z)
    end

    local units = Spring.GetUnitsInCylinder(x,z,fireRadius * fireSpeed * 1.1)
    for _,unitID in pairs(units) do
        if fireID ~= Spring.GetUnitDefID(unitID) then
            local p = ProximityInsideFire(unitID, x,y,z)
            if p > 0 then
                -- TODO: attenuation
                Spring.DestroyUnit(unitID, true, false)
            end
        end
    end
end

function gadget:GameFrame(n)
    for uID,_ in pairs(config) do
        local x,y,z = Spring.GetUnitPosition(uID)
        if x then
            UpdateFire(n,x,y,z)
        end
    end
end




-- UNSYNCED
else



end