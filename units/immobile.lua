local Barrel = BaseImmobile:New {
    name                = "Barrel",
    customParams = {
        barrel = true,
    },
    maxDamage           = 100,
}

local Fire = BaseImmobile:New {
    name                = "Fire",
    canCloak            = true,
    initCloaked         = true,
    decloakOnFire       = false,
    minCloakDistance    = -1,
    blocking            = false,
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