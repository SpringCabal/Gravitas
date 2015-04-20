local Bob = CassosRobot:New {
    name                = "B.O.B",
    objectName          = "big_robot.s3o",

    customParams = {
        invulnerable = true,
    },
    weapons = {
        { name = "BobGun"}
    },
}

local ComeToPapa = BaseRobot:New {
    name                = "ComeToPapa",
    weapons = {
        { name = "Kannon"},
        { name = "Kannon"},
        { name = "Kannon"},
    },
    mass                = 70,
    maxDamage           = 3000,
    maxVelocity         = 1.5,
    strafeToAttack      = true,
    script              = "projector_boss.lua",
	objectName          = "ComeToPapa.s3o",
}


local Wallier = CassosRobot:New {
    name                = "Wallier",
    objectName          = "med_robot.s3o",
    weapons = {
        { name = "WallesRevenge"},
    },
}


local Projector = CassosRobot:New {
    name                = "Projector",
    objectName          = "small_robot.s3o",
    weapons = {
        { name = "Kannon"},
    },
}

return lowerkeys({
    Bob            =    Bob,
    Projector      =    Projector,
    Wallier        =    Wallier,
    ComeToPapa     =    ComeToPapa,
})