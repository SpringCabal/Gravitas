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

function gadget:Initialize()
  
end

function gadget:GameStart()
   
end

function gadget:UnitCreated(unitID, unitDefID)
    if UnitDefs[unitDefID].effect then
        Spring.SetUnitNoDraw(unitID, true)
        -- Spring.SetUnitNoSelect(unitID, true) --I'm guessing this isn't wanted while we are working
    end
end

function gadget:UnitDestroyed(unitID)
   
end

-- UNSYNCED
else



end