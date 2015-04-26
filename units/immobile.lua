local Barrel = BaseImmobile:New {
    name                = "Barrel",
    customParams = {
        barrel = true,
    },
    maxDamage           = 150,
    mass                = 60,
    script             = "barrel.lua",
}

-- sci fi barrel
local Canister = BaseImmobile:New {
    name                    = "Canister",
    customParams = {
        barrel = true,
    },
    maxDamage               = 150,
    mass                    = 60,
    script                  = "canister.lua",
}

local Crate = BaseImmobile:New {
    name                    = "Crate",
    maxDamage               = 150,
    mass                    = 90,
    collisionVolumeScales   = '50 50 50',
    collisionVolumeOffsets  = '0 0 0',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'box',
    footprintX              = 5,
    footprintZ              = 5,
    script                  = "crate.lua",
    customParams = {
        plate_toggler = true,
        invulnerable = 1,
    }
}

local Gate = BaseWall:New {
    name                    = "Gate",
    script                  = "gate.lua",
    footprintX              = 20,
    footprintZ              = 4,
    collisionVolumeScales   = '400 160 50',
    collisionVolumeOffsets  = '0 56 0',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'box',
    movementClass		    = "Wall5x17", 
    customParams = {
        radius = 260,
        invulnerable = 1,
        gate   = true,
    },
    onoffable               = true,
}

local Electrafi = BaseEffect:New {
    name                = "Electrafi",
    script              = "electrafi.lua",
    customParams = {
        electric_field = true,
        invulnerable = 1,
    },
}

local Fire = BaseEffect:New {
    name                = "Fire",
    script              = "fire.lua",
    customParams = {
        fire = true,
        invulnerable = 1,
    },
}

local Plate = Unit:New {
    customParams        = {
        -- invulnerable means that most instances are invulnerable through normal damage and effects (could still be manually destroyed)
        invulnerable = true, 
        radius = 50,
    },
    footprintX			    = 8,
	footprintZ			    = 8,
	mass				    = 10e5,
    maxDamage               = 10000,
    collisionVolumeScales   = '0 0 0',
    collisionVolumeType     = 'cylY',
    pushResistant           = true,
    -- set in UnitCreated, because this isn't enough, obviously
    --blocking                = false,
    canMove                 = false, --effects cannot be moved (even by gravity)
    canGuard                = false,
    canPatrol               = false,
    canRepeat               = false,
    stealth				    = true,
	turnRate			    = 0,
	upright				    = true,
    sightDistance		    = 0,

    name                = "Plate",
    script              = "plate.lua",
    onoffable               = true,
}

local Wall1 = BaseWall:New {
    name                    = "Wall1",
    script                  = "wall1.lua",
    footprintX              = 32,
    footprintZ              = 4,
    collisionVolumeScales   = '520 160 50',
    collisionVolumeOffsets  = '0 56 0',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'box',
    customParams = {
        radius = 490,
        invulnerable = 1,
    }
}

local Wall2 = BaseWall:New {
    name                    = "Wall2",
    script                  = "wall2.lua",
    footprintX              = 17,
    footprintZ              = 4,
    collisionVolumeScales   = '260 160 50',
    collisionVolumeOffsets  = '0 56 0',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'box',
    customParams = {
        radius = 260,
        invulnerable = 1,
    }
}

local Wall3 = BaseWall:New {
    name                    = "Wall3",
    script                  = "wall3.lua",
    footprintX              = 8,
    footprintZ              = 4,
    collisionVolumeScales   = '140 160 50',
    collisionVolumeOffsets  = '0 56 0',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'box',
    customParams = {
        radius = 160,
        invulnerable = 1,
    }
}

local Wall4 = BaseWall:New {
    name                    = "Wall4",
    script                  = "wall4.lua",
    footprintX              = 5,
    footprintZ              = 4,
    collisionVolumeScales   = '70 160 50',
    collisionVolumeOffsets  = '0 56 0',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'box',
    customParams = {
        radius = 160,
        invulnerable = 1,
    }
}


return lowerkeys({
    Barrel      = Barrel,
    Canister    = Canister,
    Crate       = Crate,
    Electrafi   = Electrafi,
    Fire        = Fire,
    Gate        = Gate,
    Plate       = Plate,
    Wall1       = Wall1,
    Wall2       = Wall2,
    Wall3       = Wall3,
    Wall4       = Wall4,
})
