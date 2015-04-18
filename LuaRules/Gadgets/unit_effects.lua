function gadget:GetInfo()
  return {
    name      = "Effects gadgets",
    desc      = "Common gadgetry for effects",
    author    = "gajop",
    date      = "April 2015",
    license   = "GNU GPL, v3",
    layer     = 0,
    enabled   = true,
  }
end

if gadgetHandler:IsSyncedCode() then

function SetupUnitEffectProperties(unitID)
    Spring.SetUnitNoDraw(unitID, true)
    Spring.SetUnitNeutral(unitID, true)
--     Spring.MoveCtrl.Enable(unitID)
--     Spring.MoveCtrl.SetGravity(unitID, 0)
end

function gadget:Initialize()
    for _, unitID in pairs(Spring.GetAllUnits()) do
        local unitDefID = Spring.GetUnitDefID(unitID)
        if UnitDefs[unitDefID].customParams.effect then
            SetupUnitEffectProperties(unitID)
        end
    end
end

function gadget:GameStart()
   
end

function gadget:UnitCreated(unitID, unitDefID)
    if UnitDefs[unitDefID].customParams.effect then
        SetupUnitEffectProperties(unitID)
        -- Spring.SetUnitNoSelect(unitID, true) --I'm guessing this isn't wanted while we are working
    end
end

function gadget:UnitDestroyed(unitID)
   
end

-- UNSYNCED
else



end