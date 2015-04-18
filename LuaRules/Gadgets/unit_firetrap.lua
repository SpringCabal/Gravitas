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
end

function gadget:GameStart()
    --Spring.SendCommands("cheat")
    --Spring.SendCommands("globallos") 
end

function gadget:UnitCreated(unitID, unitDefID)
    if unitDefID == fireID then
        local x,y,z = Spring.GetUnitPosition(unitID)
        local y = Spring.GetGroundHeight(x,z)
        config[#config+1] = {x=x,y=y,z=z}
        Spring.DestroyUnit(unitID,false,true)        
    end
end

function RandomUnif(a)
    return a*(2*math.random()-1)
end

function SpawnFire(x,y,z)
    local s = fireSpeed + RandomUnif(0.1)
    local r = fireRadius + RandomUnif(fireRadius/10)
    Script.LuaRules.FlameRaw(x,y,z, 0,s,0, 0,0,0, r)
end

function ProximityInsideFire(unitID, t)
    -- return the distance inside fire cone, and 0 otherwise
    -- since we have no planes, assume that the fire extends as an infinite cone towards the sky
    if not Spring.ValidUnitID(unitID) then
        return 0
    end
    
    local x,y,z,mx,my,mz = Spring.GetUnitPosition(unitID) --should use midpos, but it seems to be broken for our units
    local r = mx and Spring.GetUnitRadius(unitID) or 0
    
    if (y<t.y) then return 0 end
    
    local nx = x - t.x
    local ny = y - t.y
    local nz = z - t.z
    local emitRotSpread = (8 / 360) * (2*math.pi)--from lups_fmale_jitter
    local baseAngle = math.tan(math.atan(emitRotSpread)/fireSpeed) -- angle between the surface of cone and the upwards normal vector
    
    local pDist = math.sqrt(nx*nx+nz*nz) -- perpendicular distance from n to central axis of cone
    local p2Dist = t.y * math.tan(baseAngle)
    local p3Dist = p2Dist - pDist -- horizontal distance from (x,y,z) to the surface of the cone
    
    if p3Dist < 0 then return 0 end
    
    local p4Dist = math.cos(baseAngle) * p3Dist -- Euclidean distance from (x,y,z) to the surface of the cone
    return p4Dist    
end

function gadget:GameFrame(n)
    if n%15==0 then
        for _,t in pairs(config) do
            SpawnFire(t.x,t.y,t.z, t.r)
        end
    end

    -- kill anything too close
    for _,t in pairs(config) do
        local units = Spring.GetUnitsInCylinder(t.x,t.z,fireRadius * fireSpeed * 1.1)
        for _,unitID in pairs(units) do
            local p = ProximityInsideFire(unitID, t)
            if p > 0 then
                -- TODO: attenuation
                Spring.DestroyUnit(unitID, true, false)
            end
        end    
    end

end




-- UNSYNCED
else



end