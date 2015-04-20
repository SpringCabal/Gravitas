local gate = piece('Gate')
local rails = piece('Rails');

function script.Create()
end

function script.Killed(recentDamage, _)
    return 1
end

local signalMask = 0

local collisionData
-- TODO: blocking/collision will be incosistent while it's pulling down/up
function script.Activate()
    StartThread(function()
        GG.AdjustWallTerrain(unitID, 0)
        Sleep(500)
        local x, y, z = Spring.GetUnitPosition(unitID)
        Spring.PlaySoundFile("sounds/gate.ogg", 0.7)
        Move(gate, z_axis, -168, 180);
        WaitForMove(gate, z_axis);
    end)
    Spring.UnitScript.Signal(signalMask)
  --  Spring.UnitScript.SetSignalMask(signalMask)
    return 1
end

function script.Deactivate()
    StartThread(function()
        Sleep(500)
        local x, y, z = Spring.GetUnitPosition(unitID)
        Spring.PlaySoundFile("sounds/gate.ogg", 0.7)
        Move(gate, z_axis, 0, 190);
        WaitForMove(gate, z_axis);
        GG.AdjustWallTerrain(unitID, 1)
    end)
    Spring.UnitScript.Signal(signalMask)
  --  Spring.UnitScript.SetSignalMask(signalMask)
    return 0
end
