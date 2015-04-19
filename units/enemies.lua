local Protector_Boss = BaseRobot:New {
    name                = "Boss",
    customParams = {
        robot = true,
    },
    weapons = {
        { name = "BossWeapon"},
    },
    mass                = 70,
    maxDamage           = 300,
    maxVelocity         = 1.5,
    strafeToAttack      = true,
    script              = "projector_boss.lua",
	objectName = "ComeToPapa.s3o",
}


local Projector_ShortRange = BaseRobot:New {
    name                = "MG Projector",
    customParams = {
        robot = true,
    },
    weapons = {
		{ name = "WallesRevenge"},
        { name = "Kannon"},
        { name = "BobGun"}       
    },
    mass                = 70,
    maxDamage           = 300,
    maxVelocity         = 1.5,
    strafeToAttack      = true,
    script              = "projector.lua",
	objectName = "big_robot.s3o",
}

local Projector_MedRange = BaseRobot:New {
    name                = "Kannon Projector",
    customParams = {
        robot = true,
    },
    weapons = {
		{ name = "WallesRevenge"},
        { name = "Kannon"},
        { name = "BobGun"}       
    },
    mass                = 70,
    maxDamage           = 300,
    maxVelocity         = 1.5,
    strafeToAttack      = true,
    script              = "projector.lua",
	objectName = "med_robot.s3o",
}


local Projector_LongRange = BaseRobot:New {
    name                = "Ray Projector",
    customParams = {
        robot = true,
    },
    weapons = {
		{ name = "WallesRevenge"},
        { name = "Kannon"},
        { name = "BobGun"}       
    },
    mass                = 70,
    maxDamage           = 300,
    maxVelocity         = 1.5,
    strafeToAttack      = true,
    script              = "projector.lua",
	objectName = "small_robot.s3o",
}



-- Could probably use the same chassis as Projector, but needs different weapons/size
local Bob = BaseRobot:New {
    name                = "B.O.B",
    customParams = {
        robot = true,
        invulnerable = true,
    },
    weapons = {
        { name = "BobGun"},
    },
    mass                = 70,
    maxDamage           = 300,
    maxVelocity         = 1.5,
    strafeToAttack      = true,
    script              = "bob.lua",
	objectName          = "projector.dae",
}


return lowerkeys({
Projector_ShortRange    =Projector_ShortRange,
Projector_LongRange     =Projector_LongRange,
Projector_MedRange		=Projector_MedRange	,
Protector_Boss		= Protector_Boss	,

 
    Bob       = Bob,
})