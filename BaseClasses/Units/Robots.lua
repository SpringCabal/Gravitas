local BaseRobot = Unit:New {
    acceleration        = 0.5,
    brakeRate           = 0.9,
    --buildCostMetal        = 65, -- used only for power XP calcs
    canMove             = true,
--     canGuard            = false,
--     canPatrol           = false,
--     canRepeat           = false,
    category            = "INFANTRY",

    pushResistant       = true,
    collisionVolumeScales   = '37 40 37',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'CylY',
    -- corpse               = "<SIDE>soldier_dead",
    footprintX          = 1,
    footprintZ          = 1,
    mass                = 50,
    maxDamage           = 300, -- default only, <SIDE>Infantry.lua should overwrite
    maxVelocity         = 1.5,
    minCollisionSpeed   = 1,
    movementClass       = "KBOT_Infantry", -- TODO: --KBOT
    radius              = 9,
    repairable          = false,
    sightDistance       = 300,


    stealth             = true,
    turnRate            = 3000,
    upright             = true,

    customparams = {
        radius = 18,
        robot  = true,
    }
}

local CassosRobot = BaseRobot:New {
    mass                = 50,
    maxDamage           = 300,
    maxVelocity         = 1.5,
    strafeToAttack      = true,
    script              = "projector.lua",
    customparams = {
        radius = 40,
    }
}

return {
    BaseRobot   = BaseRobot,
    CassosRobot = CassosRobot,
}
