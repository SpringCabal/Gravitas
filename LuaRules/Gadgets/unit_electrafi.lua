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

local elecID = UnitDefNames["electrafi"].id

function gadget:Initialize()
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
        config[unitID] = Spring.GetUnitRulesParam(unitID,"elecSize") or 300
        Spring.SetUnitNoDraw(unitID, true)
        -- Spring.SetUnitNoSelect(unitID, true) --I'm guessing this isn't wanted while we are working
        SendToUnsynced("AddShield", unitID, config[unitID])
    end
end

function gadget:UnitDestroyed(unitID)
    if config[unitID] then
        config[unitID] = nil
        SendToUnsynced("RemoveShield", unitID)
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
    -- kill stuff
    local r = config[uID] or 0
    local units = Spring.GetUnitsInSphere(x,y,z,r+250) -- +205 because its probably bigger than all unit radii
    for _,unitID in pairs(units) do
        if elecID ~= Spring.GetUnitDefID(unitID) then
            local p = ProximityInsideElec(unitID, x,y,z, r)
            if p > 0 then
                -- TODO: attenuation
                Spring.DestroyUnit(unitID, true, false)                
            end
        end
    end
end

function gadget:GameFrame(n)
    -- draw
    if n%15==0 then
        for uID,r in pairs(config) do
            config[uID] = Spring.GetUnitRulesParam(uID,"elecSize") or 300
            SendToUnsynced("SpawnElec", uID, config[uID])
        end
    end
    
    -- kill
    for uID,_ in pairs(config) do
        local x,y,z = Spring.GetUnitPosition(uID)
        if x then
            UpdateElec(n,x,y,z,uID)
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
end
function gadget:Shutdown()
    gadgetHandler:RemoveSyncAction("SpawnElec")
end

function SpawnElec(_,unitID, r)
    local x,y,z = Spring.GetUnitPosition(unitID)
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

function gadget:GameFrame(n)
    if (#particleList>0) then
        GG.Lups.AddParticlesArray(particleList)
        particleList = {}
    end
end



end