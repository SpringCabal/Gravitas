local Bob = CassosRobot:New {
    name                = "B.O.B",
    objectName          = "big_robot.s3o",
    footprintX          = 3,
    footprintZ          = 3,
    movementClass       = "BoB",
    customParams = {
        invulnerable = true,
    },
    weapons = {
        { name = "BobGun"}
    },
    strafeToAttack       = false,
}

local ComeToPapa = BaseRobot:New {
    name                = "ComeToPapa",
    footprintX          = 6,
    footprintZ          = 6,
    movementClass       = "ComeToPapa",
    weapons = {
        { name = "Kannon"},
        { name = "Kannon"},
        { name = "Kannon"},
    },
    mass                = 70,
    maxDamage           = 3000,
    maxVelocity         = 1.5,
    strafeToAttack      = false,
    script              = "projector_boss.lua",
	objectName          = "ComeToPapa.s3o",
}


local Wallier = CassosRobot:New {
    name                = "Wallier",
    objectName          = "med_robot.s3o",
    footprintX          = 3,
    footprintZ          = 3,
    movementClass       = "Wallier",
    weapons = {
        { name = "WallesRevenge"},
    },
}


local Projector = CassosRobot:New {
    name                = "Projector",
    objectName          = "small_robot.s3o",
    footprintX          = 2,
    footprintZ          = 2,
    movementClass       = "Projector",
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