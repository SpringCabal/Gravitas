local Gravit = BaseHuman:New {
    name                = "Gravit",
    activateWhenBuilt   = true,
    customParams = {
        player = true,
    },
    weapons = {
        { name = "GRAVITY_POS"},
        { name = "GRAVITY_NEG"},
    },
    maxDamage           = 800,
    maxVelocity         = 5,
    onoffable           = true,
}


return lowerkeys({
    Gravit = Gravit,
})