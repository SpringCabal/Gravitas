
function gadget:GetInfo()
  return {
    name    = "Aim sphere Fixer",
    desc    = "Sets aimi spheres to sane values",
    author  = "ashdnazg",
    date    = "",
    license = "Public Domain",
    layer   = 0,
    enabled = true,
  }
end

if (gadgetHandler:IsSyncedCode()) then 


function gadget:UnitCreated(unitID, unitDefID, unitTeam, builderID)
    local p1, p2, p3 = Spring.GetUnitCollisionVolumeData(unitID)
    if UnitDefs[unitDefID].customParams.radius then
        Spring.SetUnitRadiusAndHeight(unitID, UnitDefs[unitDefID].customParams.radius, 1)
    end
    Spring.SetUnitMidAndAimPos(unitID, 0, 25, 0, 0, 25, 0, true)
end


end