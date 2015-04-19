local gate = piece('Gate')
local rails = piece('Rails);

function script.Create()
end

function script.Killed(recentDamage, _)
    return 1
end

function script.Activate()
	Move(rails, z_axis, 0,10);
	WaitForMove(gate, z_axis);
	Move(rails, z_axis, 0, 20);
    return 1
end

function script.Deactivate()
	Move(rails, z_axis, -160,10);
	WaitForMove(gate, z_axis);
	Move(rails, z_axis, -160, 20);
    return 0
end
