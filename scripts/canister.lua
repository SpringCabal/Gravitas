local can = piece('canister')

function script.Create()
end

function script.Killed(recentDamage, _)
	Explode(can, SFX.SHATTER);
    return 1
end

function script.Activate()
    return 1
end

function script.Deactivate()
    return 0
end
