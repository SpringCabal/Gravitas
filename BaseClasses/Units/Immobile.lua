local BaseImmobile = Unit:New {
    -- movement needs to be enabled so it can be pushed by gravity
	canMove				= true,
    maxVelocity         = 0.00001,
    movementClass		= "KBOT_Infantry", 
    canGuard            = false,
    canPatrol           = false,
    canRepeat           = false,
    category            = "INFANTRY",

    pushResistant       = true,
    collisionVolumeScales   = '37 40 37',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'CylY',
	-- corpse				= "<SIDE>soldier_dead",
	footprintX			= 1,
	footprintZ			= 1,
	mass				= 50,
	maxDamage			= 300, -- default only, <SIDE>Infantry.lua should overwrite
	repairable			= false,
	sightDistance		= 250,


	stealth				= true,
	turnRate			= 3000,
	upright				= true,
}

local BaseEffect = BaseImmobile:New {
    customParams = {
        effect = true,
    },
    category     = "EFFECT",
    maxDamage    = 10000,
    blocking     = false,
    canMove      = false, --effects cannot be moved (even by gravity)
    canCloak            = true,
    initCloaked         = true,
    decloakOnFire       = false,
    minCloakDistance    = -1,
}

return {
    BaseImmobile = BaseImmobile,
    BaseEffect = BaseEffect,
}
