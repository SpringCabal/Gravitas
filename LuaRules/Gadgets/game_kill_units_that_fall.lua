function gadget:GetInfo()
  return {
    name      = "Kill units that fall down",
    desc      = "Return them to whence they came from!",
    author    = "gajop",
    date      = "April 2015",
    license   = "GNU GPL, v3",
    layer     = 0,
    enabled   = true,
  }
end

-- SYNCED
if gadgetHandler:IsSyncedCode() then

local updateRate = 31
function gadget:GameFrame()
    if Spring.GetGameFrame() % updateRate == 0 then
        local units = Spring.GetUnitsInBox(
            0, -500, 0,
            Game.mapSizeX, 50, Game.mapSizeZ)
        for _, unitID in pairs(units) do
            Spring.Echo("Killing unit that fell down", UnitDefs[Spring.GetUnitDefID(unitID)].name)
            Spring.DestroyUnit(unitID, false, true)
        end
    end
end

end