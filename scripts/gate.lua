local gate = piece('Gate')
local rails = piece('Rails');

function script.Create()
end

function script.Killed(recentDamage, _)
    return 1
end

local signalMask = 0

function script.Activate()
    StartThread(function()
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
        Move(rails, z_axis, 0, 190);
        WaitForMove(gate, z_axis);
        Move(gate, z_axis, 0, 150);
    end)
    Spring.UnitScript.Signal(signalMask)
  --  Spring.UnitScript.SetSignalMask(signalMask)
    return 0
end
