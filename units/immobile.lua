local Barrel = BaseImmobile:New {
    name                = "Barrel",
    customParams = {
        barrel = true,
    },
    maxDamage           = 150,
    mass                = 80,
    script             = "barrel.lua",
}

-- sci fi barrel
local Canister = BaseImmobile:New {
    name                    = "Canister",
    customParams = {
        barrel = true,
    },
    maxDamage               = 150,
    mass                    = 80,
    script                  = "canister.lua",
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
    script              = "crate.lua",
}

local Wall1 = BaseImmobile:New {
    name                = "Wall1",
    maxDamage           = 1500,
    customParams = {
        wall = true,
    },
    script              = "wall1.lua",
    footprintX			= 30,
	footprintZ			= 5,
}

local Fire = BaseEffect:New {
    name                = "Fire",
    customParams = {
        effect = true,
    },
    script              = "fire.lua",
}

local Electrafi = BaseEffect:New {
    name                = "Electrafi",
    customParams = {
        electric_field = true,
    },
    script              = "electrafi.lua",
}

return lowerkeys({
    Barrel      = Barrel,
    Canister    = Canister,
    Crate       = Crate,
    Fire        = Fire,
    Electrafi   = Electrafi,
    Wall1       = Wall1
})
