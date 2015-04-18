local Barrel = BaseImmobile:New {
    name                = "Barrel",
    customParams = {
        barrel = true,
    },
    maxDamage           = 100,
}

local Fire = BaseImmobile:New {
    name                = "Fire",
    blocking            = false,
    canMove             = false, --fire cannot be moved (even by gravity)
    customParams = {
        effect = true,
        fire = true,
    },
    maxDamage           = 100,
}

local Electrafi = BaseImmobile:New {
    name                = "Electrafi",
    blocking            = false,
    canMove             = false, --fire cannot be moved (even by gravity)
    customParams = {
        effect = true,
        electric_field = true,
    },
    maxDamage           = 100,
}

return lowerkeys({
    Barrel = Barrel,
    Fire   = Fire,
    Electrafi   = Electrafi,
})
