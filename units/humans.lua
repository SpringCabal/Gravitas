local Gravit = BaseHuman:New {
    name                = "Gravit",
    customParams = {
        player = true,
    },
    weapons = {
        { name = "GRAVITY_POS"},
		--{ name = "GRAVITY_NEG"},
    },
    maxDamage           = 800,
    maxVelocity        = 2,
}


return lowerkeys({
    Gravit = Gravit,
})