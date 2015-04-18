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
	-- corpse				= "",
	footprintX			= 1,
	footprintZ			= 1,
	mass				= 50,
	maxDamage			= 300, -- default only
	repairable			= false,
	sightDistance		= 0,


	stealth				= true,
	turnRate			= 3000,
	upright				= true,
}

local BaseEffect = Unit:New {
    customParams        = {
        effect = true,
    },
    category            = "EFFECT",
    footprintX			= 1,
	footprintZ			= 1,
	mass				= 50,
    maxDamage           = 10000,
    collisionVolumeScales   = '0 0 0',
    collisionVolumeType     = 'CylY',
    pushResistant       = true,
    blocking            = false,
    canMove             = false, --effects cannot be moved (even by gravity)
    canGuard            = false,
    canPatrol           = false,
    canRepeat           = false,
    stealth				= true,
	turnRate			= 0,
	upright				= true,
    sightDistance		= 0,
--     canCloak            = true,
--     initCloaked         = true,
--     decloakOnFire       = false,
--     minCloakDistance    = 0,
}

return {
    BaseImmobile = BaseImmobile,
    BaseEffect = BaseEffect,
}
