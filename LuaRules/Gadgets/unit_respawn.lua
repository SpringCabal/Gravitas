function gadget:GetInfo()
  return {
    name      = "Respawns stuff, if needed",
    desc      = "Pancake",
    author    = "Bluestone",
    date      = "May 2015",
    license   = "GNU GPL, v2 or later",
    layer     = 0,
    enabled   = true,
  }
end

local devMode = (tonumber(Spring.GetModOptions().play_mode) or 0) == 0
if devMode then return false end

-- SYNCED
if gadgetHandler:IsSyncedCode() then

local position = {} -- position[unitID]={x=,y=,z=, uDID, moved=} most recent position
local watched = {} -- watched[unitID]={x=,y=,z=} original position
local to_respawn = {}

function IsRespawnable(unitID)
    if Spring.GetUnitRulesParam(unitID,"respawnable")==1 then
        return true
    end
    return false
end

function SavePositions()
    -- save the original positions of respawnable units
    local units = Spring.GetAllUnits()
    for _,unitID in pairs(units) do
        if IsRespawnable(unitID) then
            local x,y,z = Spring.GetUnitPosition(unitID)
            position[unitID]={x=x,y=y,z=z, moved=Spring.GetGameFrame()}
            watched[unitID]={x=x,y=y,z=z, uDID=Spring.GetUnitDefID(unitID), teamID=Spring.GetUnitTeam(unitID)}
        end    
    end
end

function IHateLua(x1,y1,z1,x2,y2,z2)
    -- check equality in R3
    return (math.abs(x1-x2)+math.abs(y1-y2)+math.abs(z1-z2) > 1)
end

function CheckMoved(unitID, t, n)
    -- a unit is considered stuck if 
    -- (1) its position is not equal to where it started and it hasn't moved for 30 secs, or
    -- (2) its position is above its original height by more than 20 elmos, and it hasn't moved for 10 seconds
    -- this relies on a flat map!
        
    local x,y,z = Spring.GetUnitPosition(unitID)
    if not x then 
        to_respawn[unitID] = true
    end 
    
    local idleTime = n-position[unitID].moved
    Spring.Echo(unitID, idleTime)
    local o = watched[unitID]
    
    if IHateLua(x,y,z, t.x,t.y,t.z) then
        -- the unit has moved from its most recent position
        position[unitID].x=x
        position[unitID].y=y
        position[unitID].z=z
        position[unitID].moved = n
    elseif IHateLua(x,y,z, o.x,o.y,o.z) then
        -- the unit has moved from its original position
        if idleTime>30*30 or (idleTime>10*30 and y>20+o.y) then
            -- the unit is stuck
            to_respawn[unitID] = true        
        end    
    end
end

function Respawn(unitID)
    Spring.Echo("rs", unitID)
    -- erase from our tables
    local unitDefID = watched[unitID].uDID
    local x,y,z = watched[unitID].x,watched[unitID].y,watched[unitID].z
    local teamID = watched[unitID].teamID
    watched[unitID] = nil
    position[unitID] = nil
    
    -- remove the old unit, if it still exists
    if Spring.ValidUnitID(unitID) then
        Spring.DestroyUnit(unitID,false,false)
    end
    
    -- make a green flash in the removal and the respawn positions
    -- TODO
    
    -- spawn the new unit, update our tables
    local newUnitID = Spring.CreateUnit(unitDefID, x,y,z, "n", teamID)
    position[newUnitID]={x=x,y=y,z=z, moved=Spring.GetGameFrame()}
    watched[newUnitID]={x=x,y=y,z=z, uDID=Spring.GetUnitDefID(newUnitID), teamID=Spring.GetUnitTeam(newUnitID)}
    Spring.SetUnitRulesParam(newUnitID,"respawnable",1)
end

function gadget:UnitDestroyed(unitID, unitDefID, unitTeam, attackerID, attackerDefID, attackerTeam)
    if watched[unitID] then
        to_respawn[unitID] = true
    end
end
    
function gadget:Initialize()
    SavePositions() -- handle luarules reload
end    

function gadget:GameFrame(n)
    if n==30 then SavePositions() end
    
    if n%30==0 then
        -- update position
        for unitID,t in pairs(position) do
            CheckMoved(unitID, t, n)
        end
        
        -- respawn 
        for unitID,_ in pairs(to_respawn) do
            Respawn(unitID)
        end
        to_respawn = {}
    end
end
    

end