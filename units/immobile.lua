local Barrel = BaseImmobile:New {
    name                = "Barrel",
    customParams = {
        player = true,
    },
    maxDamage           = 100,
}


return lowerkeys({
    Barrel = Barrel,
})