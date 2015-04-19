local Bob = CassosRobot:New {
    name                = "B.O.B",
    objectName          = "big_robot.s3o",
    customParams = {
        invulnerable = true,
    },
}

local Wallier = CassosRobot:New {
    name                = "Wallier",
    objectName          = "med_robot.s3o",
}


local Projector = CassosRobot:New {
    name                = "Projector",
    objectName          = "small_robot.s3o",
}

return lowerkeys({
    Bob           =    Bob,
    Projector     =    Projector,
    Wallier       =    Wallier,
})