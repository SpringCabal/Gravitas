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
    fireState           = 0,
}


return lowerkeys({
    Gravit = Gravit,
})