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
        local x, y, z = Spring.GetUnitPosition(unitID)
        Spring.PlaySoundFile("sounds/gate.ogg", 0.7, x, y, z)
        Spring.SetUnitBlocking(unitID, false)
        collisionData = {Spring.GetUnitCollisionVolumeData(unitID)}
        Spring.SetUnitCollisionVolumeData(unitID,
            0, 0, 0,
            0, 0, 0,
            0, 0, 0
        )
        Move(rails, z_axis, -150, 180);
        WaitForMove(gate, z_axis);
        Move(gate, z_axis, -150, 150);
    end)
    Spring.UnitScript.Signal(signalMask)
  --  Spring.UnitScript.SetSignalMask(signalMask)
    return 1
end

function script.Deactivate()
    StartThread(function()
        local x, y, z = Spring.GetUnitPosition(unitID)
        Spring.PlaySoundFile("sounds/gate.ogg", 0.7, x, y, z)
        Spring.SetUnitBlocking(unitID, true)
        if collisionData ~= nil then
            Spring.SetUnitCollisionVolumeData(unitID, unpack(collisionData))
            collisionData = nil
        end
        Move(rails, z_axis, 0, 190);
        WaitForMove(gate, z_axis);
        Move(gate, z_axis, 0, 150);
    end)
    Spring.UnitScript.Signal(signalMask)
  --  Spring.UnitScript.SetSignalMask(signalMask)
    return 0
end
