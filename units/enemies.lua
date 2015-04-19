local Bob = BaseRobot:New {
    name                = "B.O.B",
    objectName          = "big_robot.s3o",
    script              = "projector.lua",
    customParams = {
        invulnerable = true,
    },
}

local Wallier = BaseRobot:New {
    name                = "Wallier",
    objectName          = "med_robot.s3o",
    script              = "projector.lua",
}


local Projector = BaseRobot:New {
    name                = "Projector",
    objectName          = "small_robot.s3o",
    script              = "projector.lua",
}

return lowerkeys({
    Bob           =    Bob,
    Projector     =    Projector,
    Wallier       =    Wallier,
})