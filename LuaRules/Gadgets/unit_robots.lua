function gadget:GetInfo()
  return {
    name      = "Robot mechanics",
    desc      = "No-friendly fire, try to avoid walking into things that kill you",
    author    = "gajop, Bluestone",
    date      = "April 2015",
    license   = "GNU GPL, v3",
    layer     = 0,
    enabled   = true,
  }
end

-- SYNCED
if gadgetHandler:IsSyncedCode() then

local fireID = UnitDefNames["fire"].id 
local elecID = UnitDefNames["electrafi"].id

local movingAway = {} -- unitIDs of robots that are moving away from inaminate objects that attacked them

function IsInaminateObject(unitDefID)
    return (unitDefID and UnitDefs[unitDefID].customParams) and UnitDefs[unitDefID].customParams.effect or false
end

function GetInanimateObjectAttackRadius(unitID)
    if not Spring.ValidUnitID(unitID) then return 0 end
    local uDID = Spring.GetUnitDefID(unitID)
    local r = 100 --default
    if uDID==fireID then 
        r = Spring.GetUnitRulesParam(unitID,"fireSize")
    elseif uDID==elecID then
        Spring.GetUnitRulesParam(unitID,"elecSize")
    end
    return r
end

function IsInMap(x,z)
    if x<0 or x>Game.mapSizeX then return false end    
    if z<0 or z>Game.mapSizeZ then return false end
    return true
end

function SafeLocationScore(x,z,toAvoid)
    -- score +/- 1 for each object that this point is range of
    local score = 0
    for _,t in ipairs(toAvoid) do
        local d = math.sqrt((x-t.x)*(x-t.x)+(z-t.z)*(z-t.z))
        score = score + ((d>t.r) and 1 or -3) -- magic numbers controlling preference for bad/good points
    end
    return score
end

function GetSafestLocation(unitID,x,y,z,toAvoid)
    -- we search over radial paths from where we currently sit, trying to find a clear direction to move in
    -- assume we are on flat-ish ground
    local moveDist = 120 
    local checkDist = moveDist*1.1
    local circleDivs = 12
    local lineDivs = 6
    local maxDeltaH = 10
    
    local goodPoints = {} -- array table, format of element is {x=,y=,z=,score=, OK=}
    local bestPoint = {x=-1,y=-1,z=-1,score=-math.huge, OK=false}
    
    -- choose our initial theta to be perpendicular to our current direction (for symmetry)
    local closestObstacleDist = 10e10
    local theta = 0
    local rotation_dir = 1
    for _,t in ipairs(toAvoid) do
        local d = math.sqrt((x-t.x)*(x-t.x)+(z-t.z)*(z-t.z))
        if d<closestObstacleDist and d>0 then
            closestObstacleDist = d
            theta = math.acos((x-t.x)/d)
            if (t.z>z) then theta = -theta end --z is downwards, x is rightwards
        end
    end
    
    -- now, search
    for n=1,circleDivs do 
        local score = 0
    
        local vec_x = math.cos(theta)
        local vec_z = math.sin(theta) 
        local ex = x + moveDist * vec_x
        local ez = z + moveDist * vec_z
        local ey = Spring.GetGroundHeight(ex,ez)
        
        
        if math.abs(y-ey)>maxDeltaH then score = score - 10e10 end -- don't even think about it unless its flat
        if not IsInMap(ex,ez) then score = score - 10e10 end
        
        for m=1,lineDivs do
            local mx = x + checkDist * vec_x * (m/lineDivs)
            local mz = z + checkDist * vec_z * (m/lineDivs)
            score = score + SafeLocationScore(mx,mz,toAvoid)
        end
                
        local thisPoint = {x=ex,y=ey,z=ez, score=score, OK=false}  
        --Spring.MarkerAddPoint(ex,ey,ez,tostring(n) .. " " .. tostring(score))
        
        if #goodPoints==0 or score>=goodPoints[1].score then            
            if #goodPoints>1 and score>goodPoints[1].score then goodPoints = {} end
            thisPoint.OK = true
            goodPoints[#goodPoints+1] = thisPoint
        end
        
        theta = theta + (2*math.pi / circleDivs)
    end
    
    -- search through the goodPoints for the one that best matches our current direction
    local sx,_,sz = Spring.GetUnitVelocity(unitID)
    local bestMatch = 10e10
    for n,t in ipairs(goodPoints) do
        local angleDif = math.atan2(t.x-x,t.z-z)-math.atan2(sx,sz)
        if math.abs(angleDif) < bestMatch then
            bestMatch = math.abs(angleDif)
            bestPoint = t
        end    
    end
       
    return bestPoint
end

function gadget:UnitPreDamaged(unitID, unitDefID, unitTeam, damage, paralyzer, weaponDefID, projectileID, attackerID, attackerDefID, attackerTeam)
    if not unitDefID then return damage,1.0 end
    if not attackerDefID then return damage,1.0 end
    
    -- no friendly fire
    if UnitDefs[unitDefID].customParams.robot and UnitDefs[attackerDefID].customParams.robot and unitTeam == attackerTeam then
        return 0,0
    end
    
    -- try not to walk into things that kill you x_x
    local h,mh,paralyze = Spring.GetUnitHealth(unitID)
    local isParalyzed = (mh and paralyze) and (paralyze >= mh) or false
    local thisFrame = Spring.GetGameFrame() or 0
    local isBusyMovingAway = movingAway[unitID] and (movingAway[unitID] + 30 > thisFrame) -- delay to prevent order spam
    if UnitDefs[unitDefID].customParams.robot and (not isParalyzed) and IsInaminateObject(attackerDefID) and (not isBusyMovingAway) then
        -- try to walk away from it
        local x,y,z = Spring.GetUnitPosition(unitID)
        local ax,ay,az = Spring.GetUnitPosition(attackerID)        
        
        if x and ax then
            -- make a list of things to avoid
            local ar = GetInanimateObjectAttackRadius(attackerID, attackerDefID)
            local d = math.sqrt((x-ax)*(x-ax)+(y-ax)*(y-ay)+(z-az)*(z-az))
            local units = Spring.GetUnitsInSphere(x,y,z,math.max(ar,d)*1.5)
            local thingsToAvoid = {}
            for _,avoidID in pairs(units) do
                local uDID = Spring.GetUnitDefID(avoidID)
                local px,py,pz = Spring.GetUnitPosition(avoidID)
                if IsInaminateObject(uDID) then thingsToAvoid[#thingsToAvoid+1] = {x=px,y=py,z=pz, r=GetInanimateObjectAttackRadius(avoidID)}  end
            end
            
            -- work out how to avoid them
            local best = GetSafestLocation(unitID,x,y,z,thingsToAvoid)
            
            -- give/replace move order
            if best.OK then
                local Q = Spring.GetUnitCommands(unitID,1)
                local hasOrderQ = #(Q) > 0
                if hasOrderQ and movingAway[unitID] then
                    Spring.GiveOrderToUnit(unitID, CMD.REMOVE, {Q[1].tag}, {""}) -- erase previous move away order
                end
                Spring.GiveOrderToUnit(unitID, CMD.INSERT, {0, CMD.MOVE, 0, best.x, best.y, best.z}, {"alt"})
                movingAway[unitID] = Spring.GetGameFrame()
            end
        end    
    end
    
    return damage, 1.0
end

function gadget:UnitCmdDone(unitID)
    movingAway[unitID] = nil
end

end