local Gravit = BaseHuman:New {
    name                = "Gravit",
    activateWhenBuilt   = true,
    customParams = {
        player = true,
        radius = 40,
    },
    weapons = {
        { name = "GRAVITY_POS"},
        { name = "GRAVITY_NEG"},
    },
    maxDamage           = 800,
    maxVelocity         = 5,
    onoffable           = true,
    fireState           = 0,
    moveState           = 0,
    script              = "gravit.lua",
}


return lowerkeys({
    Gravit = Gravit,
})
