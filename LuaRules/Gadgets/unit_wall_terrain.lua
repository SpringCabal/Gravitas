
function gadget:GetInfo()
  return {
    name    = "Wall Terrain Maker",
    desc    = "Sets terrain height below walls",
    author  = "ashdnazg",
    date    = "",
    license = "Public Domain",
    layer   = 0,
    enabled = true,
  }
end

if (gadgetHandler:IsSyncedCode()) then 

local function AdjustWallTerrain(unitID, height)
    
    local wx, wy, wz = Spring.GetUnitPosition(unitID)
    local scaleX, scaleY, scaleZ, offsetX, offsetY, offsetZ = Spring.GetUnitCollisionVolumeData(unitID)
    local dx, dy, dz = Spring.GetUnitDirection(unitID)
    local x1 = wx - ((scaleX * dz / 2) + scaleZ * dx / 2)
    local z1 = wz + scaleX * dx / 2 -- ((scaleZ * dz / 2) - scaleX * dx / 2)
    local x2 = wx + ((scaleX * dz / 2) + scaleZ * dx / 2)
    local z2 = wz - scaleX * dx / 2 -- ((scaleZ * dz / 2) - scaleX * dx / 2)
    
    --Spring.Echo(wx, wy, wz, dx, dy, dz, x1, z1, x2, z2, scaleZ)
    
    local x, z = x1, z1
    local xdiff = dz * 8
    local zdiff = -(dx * 8)
    local num
    if math.abs(xdiff) > math.abs(zdiff) then
        num = math.ceil(math.abs((x1 - x2) / xdiff)) + 1
    else
        num = math.ceil(math.abs((z1 - z2) / zdiff)) + 1
    end
    --local rows = math.floor(math.abs(z1 - z2) / zdiff)
    --Spring.Echo(num, xdiff, zdiff)
    -- local minx, maxx, minz, maxz = math.min(x1, x2), math.max(x1, x2), math.min(z1, z2), math.max(z1, z2)
    Spring.Echo(x, z, xdiff, zdiff, num)
    for i = 1, num do
        Spring.LevelHeightMap(x - 4, z - 4, x + 4, z + 4, wy + height * scaleY)
        x = x + xdiff
        z = z + zdiff
    end
end


function gadget:UnitCreated(unitID, unitDefID, unitTeam, builderID)
    local unitDef = UnitDefs[unitDefID]
    if not unitDef.customParams.wall then return end
    --local dx, dy, dz = Spring.SetUnitRotation(unitID, 0, , 0)
    GG.Delay.DelayCall(AdjustWallTerrain, {unitID, 1})
end

function gadget:UnitDestroyed(unitID, unitDefID, unitTeam, builderID)
    local unitDef = UnitDefs[unitDefID]
    if not unitDef.customParams.wall then return end
    GG.Delay.DelayCall(AdjustWallTerrain, {unitID, 0})
end


end