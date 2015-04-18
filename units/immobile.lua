local Barrel = BaseImmobile:New {
    name                = "Barrel",
    customParams = {
        barrel = true,
    },
    maxDamage           = 100,
}

local Fire = BaseImmobile:New {
    name                = "Fire",
    canMove             = false, --fire cannot be moved (even by gravity)
    customParams = {
        fire = true,
    },
    maxDamage           = 100,
}

return lowerkeys({
    Barrel = Barrel,
    Fire   = Fire,
})