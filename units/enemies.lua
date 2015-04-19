local Bob = BaseRobot:New {
    name                = "B.O.B",
    objectName          = "big_robot.s3o",
    customParams = {
        invulnerable = true,
    },
}

local Wallier = BaseRobot:New {
    name                = "Wallier",
    objectName          = "med_robot.s3o",
}


local Projector = BaseRobot:New {
    name                = "Projector",
    objectName          = "small_robot.s3o",
}

return lowerkeys({
    Bob           =    Bob,
    Projector     =    Projector,
    Wallier       =    Wallier,
})