local BaseHuman = Unit:New {
    acceleration        = 0.5,
    brakeRate           = 0.9,
    --buildCostMetal        = 65, -- used only for power XP calcs
    canMove             = true,
--     canGuard            = false,
--     canPatrol           = false,
--     canRepeat           = false,
    category            = "INFANTRY",

    --pushResistant       = true,
    collisionVolumeScales   = '37 40 37',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'CylY',
    -- corpse               = "<SIDE>soldier_dead",
    footprintX          = 2,
    footprintZ          = 2,
    mass                = 50,
    maxDamage           = 300, -- default only, <SIDE>Infantry.lua should overwrite
    maxVelocity         = 1.5,
    minCollisionSpeed   = 1,
    movementClass       = "KBOT_Infantry", -- TODO: --KBOT
    repairable          = false,
    sightDistance       = 800,


    stealth             = true,
    turnRate            = 3000,
    upright             = true,
}

return {
    BaseHuman = BaseHuman,
}
