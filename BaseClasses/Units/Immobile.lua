local BaseImmobile = Unit:New {
	--buildCostMetal		= 65, -- used only for power XP calcs
	canMove				= false,
--     canGuard            = false,
--     canPatrol           = false,
--     canRepeat           = false,
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

return {
    BaseImmobile = BaseImmobile,
}
