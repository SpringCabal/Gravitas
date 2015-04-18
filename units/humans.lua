local Gravit = BaseHuman:New {
    name                = "Gravit",
    customParams = {
        player = true,
    },
--     weapons = {
--         { name = "Staff"},
--     },
    maxDamage           = 800,
    maxVelocity        = 2,
}


return lowerkeys({
    Gravit = Gravit,
})