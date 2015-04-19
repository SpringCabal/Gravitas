local Projector = BaseRobot:New {
    name                = "Projector",
    customParams = {
        robot = true,
    },
    weapons = {
        { name = "Kannon"},
    },
    mass                = 70,
    maxDamage           = 300,
    maxVelocity         = 1.5,
    strafeToAttack      = true,
    script              = "projector.lua",
	objectName = "evilrobot.s3o",
}

-- Could probably use the same chassis as Projector, but needs different weapons/size
local Bob = BaseRobot:New {
    name                = "B.O.B",
    customParams = {
        robot = true,
        invulnerable = true,
    },
    weapons = {
        { name = "BobGun"},
    },
    mass                = 70,
    maxDamage           = 300,
    maxVelocity         = 1.5,
    strafeToAttack      = true,
    script              = "bob.lua",
	objectName          = "projector.dae",
}


return lowerkeys({
    Projector = Projector,
    Bob       = Bob,
})