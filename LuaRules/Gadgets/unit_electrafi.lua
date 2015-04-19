function gadget:GetInfo()
  return {
    name      = "Electric Fields",
    desc      = "Stuns robots, kills humans",
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
local currentFrame 

local elecUnits = {} -- elecUnits[unitID]={elecFrame=, proximity=}
local elecRobots = {}
local burnTime = 60

local elecID = UnitDefNames["electrafi"].id

function gadget:Initialize()
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
    if unitDefID == elecID then
        if Spring.GetUnitRulesParam(unitID, "elecSize") == nil then
           Spring.SetUnitRulesParam(unitID, "elecSize", 100)
        end
        config[unitID] = Spring.GetUnitRulesParam(unitID,"elecSize")
        Spring.SetUnitNoDraw(unitID, true)
        -- Spring.SetUnitNoSelect(unitID, true) --I'm guessing this isn't wanted while we are working
    end
end

function gadget:UnitDestroyed(unitID)
    if config[unitID] then
        config[unitID] = nil
    end
end

function ProximityInsideElec(unitID, fx,fy,fz, r)
    -- return the distance inside the field
    -- assume no planes
    if not Spring.ValidUnitID(unitID) then
        return 0
    end
    
    local x,y,z,mx,my,mz = Spring.GetUnitPosition(unitID) --should use midpos, but it seems to be broken for our units
    local mr = mx and Spring.GetUnitRadius(unitID) or 0
    
    local d = (x-fx)*(x-fx)+(y-fy)*(y-fy)+(z-fz)*(z-fz)
    if d <= mr*mr + r*r then
        return r - (math.sqrt(d) - mr)
    end
    return 0
end

function UpdateElec(n, x,y,z, uID)
    -- kill/paralyse stuff
    local r = config[uID] or 0
    local units = Spring.GetUnitsInSphere(x,y,z,r+250) -- +250 because its probably bigger than all unit radii
    for _,unitID in pairs(units) do
        local unitDef = UnitDefs[Spring.GetUnitDefID(unitID)]
        local p = ProximityInsideElec(unitID, x,y,z, r)
        if p>0 then
            if unitDef.customParams.player then 
                elecUnits[unitID] = {elecFrame = currentFrame, proximity = p}
            elseif unitDef.customParams.robot then
                elecRobots[unitID] = {elecFrame = currentFrame, proximity = p}                
            end
        elseif elecUnits[unitID] then
            elecUnits[unitID].proximity = p
        elseif elecRobots[unitID] then
            elecRobots[unitID].proximity = p
        end
    end
end

function gadget:GameFrame(n)
    currentFrame = n

    -- draw
    if n%15==0 then
        for uID,r in pairs(config) do
            config[uID] = Spring.GetUnitRulesParam(uID,"elecSize") or 200
            SendToUnsynced("SpawnElec", uID, config[uID])
        end
    end
    
    -- update
    for uID,_ in pairs(config) do
        local x,y,z = Spring.GetUnitPosition(uID)
        if x then
            UpdateElec(n,x,y,z,uID)
        end
    end
    
    --kill
    for unitID,t in pairs(elecUnits) do
        if t.proximity > 0 then
            local numSparks = 1
            local intensity = 1.0
            SendToUnsynced("SpawnSpark", unitID, intensity)
            Spring.AddUnitDamage(unitID, 2+0.1*t.proximity)                
        elseif (t.elecFrame+burnTime>currentFrame) then
            local intensity = (t.elecFrame+burnTime - currentFrame) / burnTime
            SendToUnsynced("SpawnSpark", unitID, intensity)
            Spring.AddUnitDamage(unitID, 2)        
        else
            elecUnits[unitID] = nil
        end
    end
    
    -- paralyse
    for unitID,t in pairs(elecRobots) do
        if t.proximity > 0 then
            local intensity = 1.0
            SendToUnsynced("SpawnSpark", unitID, intensity)
            Spring.AddUnitDamage(unitID, 2+0.1*t.proximity, 1)
        elseif (t.elecFrame+burnTime>currentFrame) then
            local intensity = (t.elecFrame+burnTime - currentFrame) / burnTime
            SendToUnsynced("SpawnSpark", unitID, intensity)
        else
            elecUnits[unitID] = nil
        end
    end
end

function gadget:UnitPreDamaged(unitID, unitDefID, unitTeam, damage, paralyzer, weaponDefID, projectileID, attackerID, attackerDefID, attackerTeam)
    if unitDefID==elecID then return 0,0 end
    return damage,1.0
end



-- UNSYNCED
else
--

local particleList = {}

function gadget:Initialize()
    gadgetHandler:AddSyncAction("SpawnElec", SpawnElec)
    gadgetHandler:AddSyncAction("SpawnSpark", SpawnSpark)
end
function gadget:Shutdown()
    gadgetHandler:RemoveSyncAction("SpawnElec")
    gadgetHandler:RemoveSyncAction("SpawnSpark")
end

function SpawnElec(_,unitID, r)
    local x,y,z = Spring.GetUnitPosition(unitID)
    local sx,sy,sz = Spring.GetUnitVelocity(unitID)
    local partpos = "x*delay,y*delay,z*delay|x=0,y=0,z=0"
    
    particleList[#particleList+1] = {
      class        = 'JitterParticles2',
      colormap     = { {0.1,0.1,0.1,0.1},{0.2,0.2,0.2,0.01} },
      count        = 1,
      life         = r,
      delaySpread  = 0,
      force        = {0,1,0},

      partpos      = partpos,
      pos          = {x,y,z},

      emitVector   = {0,0.2,0},
      emitRotSpread= 20,

      speed        = 10,
      speedSpread  = 1.0,
      speedExp     = 1.5,

      size         = 5,
      sizeGrowth   = 10.0,

      scale        = 1.5,
      strength     = 0.25,
      heat         = 0.75,
    }

    particleList[#particleList+1] = {
      class        = 'SimpleParticles2',
      colormap     = { {0.5, 0.5, 1, 0.01},
                       {0.7, 0.7, 1, 0.01},
                       {0.5, 0.5, 0.75, 0.01},
                       {0.15, 0.15, 0.35, 0.25}, 
                       {0.025, 0.005, 0.1, 0.2},
                       {0, 0, 0.1, 0} },
      count        = 2,
      life         = r,
      delaySpread  = 25,

      force        = {0,1,0},
      --forceExp     = 0.2,

      partpos      = partpos,
      pos          = {x,y,z},

      emitVector   = {0,0.1,0},
      emitRotSpread= 16,

      rotSpeed     = 1,
      rotSpread    = 360,
      rotExp       = 9,

      speed        = 10,
      speedSpread  = 0,
      speedExp     = 1.5,

      size         = 2,
      sizeGrowth   = 4.0,
      sizeExp      = 0.7,

      texture     = "bitmaps/GPL/electricity.png",
    }    
end

function UnifRand(a)
    return a * (2*math.random() - 1)
end

function SpawnSpark(_,unitID,intensity)
    local r = Spring.GetUnitRadius(unitID)
    if not r then return end -- some models seems to have broken radii
    local x,y,z = Spring.GetUnitPosition(unitID)
    local x = x + r/2 * UnifRand(1)
    local y = y + r/2 * UnifRand(1)
    local z = z + r/2 * UnifRand(1)
    local sx,sy,sz = Spring.GetUnitVelocity(unitID)
    local partpos = "x*delay,y*delay,z*delay|x=0,y=0,z=0"
    
    local life = (15 + 7 * UnifRand(1)) * intensity
    local sizeGrowth = 7.0 + 2.0 * UnifRand(1)
    
    particleList[#particleList+1] = {
      class        = 'JitterParticles2',
      colormap     = { {0.1,0.1,0.1,0.1},{0.2,0.2,0.2,0.01} },
      count        = 1,
      life         = life,
      delaySpread  = 0,
      force        = {sx*0.5,0.6,sz*0.5},

      partpos      = partpos,
      pos          = {x,y,z},

      emitVector   = {0,0.2,0},
      emitRotSpread= 20,

      speed        = 10,
      speedSpread  = 1.0,
      speedExp     = 1.5,

      size         = 5,
      sizeGrowth   = sizeGrowth + 2.0,

      scale        = 1.5,
      strength     = 0.25,
      heat         = 0.75,
    }

    particleList[#particleList+1] = {
      class        = 'SimpleParticles2',
      colormap     = { {0.5, 0.5, 1, 0.01},
                       {0.7, 0.7, 1, 0.01},
                       {0.5, 0.5, 0.75, 0.01},
                       {0.15, 0.15, 0.35, 0.25}, 
                       {0.025, 0.005, 0.1, 0.2},
                       {0, 0, 0.1, 0} },
      count        = 2,
      life         = life,
      delaySpread  = 0,

      force        = {sx*0.65,0.5,sz*0.65},
      --forceExp     = 0.2,

      partpos      = partpos,
      pos          = {x,y,z},

      emitVector   = {0,0.1,0},
      emitRotSpread= 16,

      rotSpeed     = 1,
      rotSpread    = 360,
      rotExp       = 9,

      speed        = 10,
      speedSpread  = 0,
      speedExp     = 1.5,

      size         = 2,
      sizeGrowth   = sizeGrowth,
      sizeExp      = 1.3,

      texture     = "bitmaps/GPL/electricity.png",
    }    
end

function gadget:GameFrame(n)
    if (#particleList>0) then
        GG.Lups.AddParticlesArray(particleList)
        particleList = {}
    end
end


end