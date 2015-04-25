function gadget:GetInfo()
  return {
    name      = "Unselectable objects",
    desc      = "Makes some objects unselectable (during play mode)",
    author    = "gajop",
    date      = "April 2015",
    license   = "GNU GPL, v3",
    layer     = 0,
    enabled   = true,
  }
end

-- SYNCED
if gadgetHandler:IsSyncedCode() then
    
local devMode = (tonumber(Spring.GetModOptions().play_mode) or 0) == 0

function gadget:UnitCreated(unitID, unitDefID)
    if not devMode then
        local unitDef = UnitDefs[unitDefID]
        if unitDef.customParams.wall or unitDef.customParams.effect or unitDef.name == "plate" then
            Spring.SetUnitNoSelect(unitID, true)
        end
    end
end

end