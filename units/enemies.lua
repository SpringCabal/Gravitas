local Projector = BaseRobot:New {
    name                = "Projector",
    customParams = {
        robot = true,
    },
    weapons = {
        { name = "Kannon"},
    },
    maxDamage           = 300,
    maxVelocity         = 1.5,
    strafeToAttack      = true,
    script              = "projector.lua",
	objectName = "evilrobot.s3o",
}


return lowerkeys({
    Projector = Projector,
})