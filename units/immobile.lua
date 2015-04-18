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
	shield = {
        alpha = 0.5,
        badcolor = {1, 0.2, 0.2},
        energyuse = 0,
        force = 0.0,
        goodcolor = {0.2, 1, 0.2},
        intercepttype = 1,
        --maxspeed = 500,
        power = 10000,
        startingpower = 10000,
        powerregen = 10000,
        powerregenenergy = 0,
        radius = 100,
        -- rechargedelay = 1,
        repulser = false,
        smart = true,
        visiblerepulse = false,
        visible = true,
    },
	weapontype = "Shield",
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
