local frags = {}

for i=1,11 do
    frags[i] = piece('frag'..i);
end

function script.Create()
end

function script.Killed(recentDamage, _)
    for i,v in ipairs(frags) do
		local x,y,z = Spring.GetUnitPiecePosDir(unitID, v);
		Spring.SpawnCEG("flashnuke", x,y,z, 0,0,0, 0, 0)
		Explode(v, SFX.FALL+SFX.EXPLODE);
    end
    
    return 1
end

function script.Activate()
    return 1
end

function script.Deactivate()
    return 0
end
