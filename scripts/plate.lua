local button = piece('button')
local middle = piece('middle')
local base = piece('base')

function script.Create()
end

function script.Killed(recentDamage, _)
    return 1
end

local signalMask = 1

-- TODO: blocking/collision will be incosistent while it's pulling down/up
function script.Activate()
    Spring.UnitScript.Signal(signalMask)
    StartThread(function()
        Spring.UnitScript.SetSignalMask(signalMask)
        Spring.PlaySoundFile("sounds/click.ogg", 1)
        Move(button, z_axis, -15, 150);
        WaitForMove(button, z_axis);
        Move(middle, z_axis, -5, 50);
    end)
    return 1
end

function script.Deactivate()
    Spring.UnitScript.Signal(signalMask)
    StartThread(function()
        Spring.UnitScript.SetSignalMask(signalMask)
        Spring.PlaySoundFile("sounds/click.ogg", 1)
        Move(button, z_axis, 0, 150);
        WaitForMove(button, z_axis);
        Move(middle, z_axis, 0, 50);
    end)
    return 0
end