local Barrel = BaseImmobile:New {
    name                = "Barrel",
    customParams = {
        barrel = true,
    },
    maxDamage           = 100,
}


return lowerkeys({
    Barrel = Barrel,
})