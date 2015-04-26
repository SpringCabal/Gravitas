local Bob = CassosRobot:New {
    name                = "B.O.B",
    objectName          = "big_robot.s3o",
    footprintX          = 3,
    footprintZ          = 3,
    collisionVolumeScales   = '60 60 58',
    collisionVolumeOffsets	= '0 2 4',
    collisionVolumeTest	    = 1,
    collisionVolumeType	    = 'CylZ',
    movementClass       = "BoB",
    customParams = {
        invulnerable = true,
        radius = 30,
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
    collisionVolumeScales	= '105 104 172',
    collisionVolumeOffsets	= '0 0 -17',
    collisionVolumeTest	    = 1,
    collisionVolumeType	    = 'CylZ',    
    movementClass       = "ComeToPapa",
    customParams = {
        radius = 70,
    },
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
    collisionVolumeScales	= '52 53 52',
    collisionVolumeOffsets	= '0 4 4',
    collisionVolumeTest	    = 1,
    collisionVolumeType	    = 'CylY',    
    movementClass       = "Wallier",
    customParams = {
        radius = 27,
    },
    weapons = {
        { name = "WallesRevenge"},
    },
}


local Projector = CassosRobot:New {
    name                = "Projector",
    objectName          = "small_robot.s3o",
    footprintX          = 2,
    footprintZ          = 2,
    collisionVolumeScales   = '30 29 39',
    collisionVolumeOffsets	= '0 -5 3',
    collisionVolumeTest	    = 1,
    collisionVolumeType	    = 'CylZ',    
    movementClass       = "Projector",
    customParams = {
        radius = 15,
    },
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