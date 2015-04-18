local Barrel = BaseImmobile:New {
    name                = "Barrel",
    customParams = {
        barrel = true,
    },
    maxDamage           = 150,
    mass                = 80,
}

-- sci fi barrel
local Canister = BaseImmobile:New {
    name                    = "Canister",
    customParams = {
        barrel = true,
    },
    maxDamage               = 150,
    mass                    = 80,
}

local Crate = BaseImmobile:New {
    name                = "Crate",
    maxDamage           = 150,
    mass                = 100,
    collisionVolumeScales   = '50 50 50',
    collisionVolumeOffsets  = '0 0 0',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'box',
    footprintX			= 5,
	footprintZ			= 5,
}

local Wall1 = BaseImmobile:New {
    name                = "Wall1",
    maxDamage           = 1500,
    customParams = {
        wall = true,
    }
}

local Fire = BaseEffect:New {
    name                = "Fire",
    customParams = {
        effect = true,
    },
}

local Electrafi = BaseEffect:New {
    name                = "Electrafi",
    customParams = {
        electric_field = true,
    },
}

return lowerkeys({
    Barrel      = Barrel,
    Canister    = Canister,
    Crate       = Crate,
    Fire        = Fire,
    Electrafi   = Electrafi,
    Wall1       = Wall1
})
