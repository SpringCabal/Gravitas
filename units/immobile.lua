local Barrel = BaseImmobile:New {
    name                = "Barrel",
    customParams = {
        barrel = true,
    },
    maxDamage           = 150,
}

-- sci fi barrel
local Canister = BaseImmobile:New {
    name                = "Canister",
    customParams = {
        barrel = true,
    },
    maxDamage           = 150,
}

local Crate = BaseImmobile:New {
    name                = "Crate",
    maxDamage           = 150,
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
