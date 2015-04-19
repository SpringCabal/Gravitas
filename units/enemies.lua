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
    Bob           =    Bob,
    Projector     =    Projector,
    Wallier       =    Wallier,
})