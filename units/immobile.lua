local Barrel = BaseImmobile:New {
    name                = "Barrel",
    customParams = {
        barrel = true,
    },
    maxDamage           = 150,
    mass                = 80,
    script             = "barrel.lua",
}

-- sci fi barrel
local Canister = BaseImmobile:New {
    name                    = "Canister",
    customParams = {
        barrel = true,
    },
    maxDamage               = 150,
    mass                    = 80,
    script                  = "canister.lua",
}

local Crate = BaseImmobile:New {
    name                    = "Crate",
    maxDamage               = 150,
    mass                    = 100,
    collisionVolumeScales   = '50 50 50',
    collisionVolumeOffsets  = '0 0 0',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'box',
    footprintX              = 5,
    footprintZ              = 5,
    script                  = "crate.lua",
}

local Gate = BaseWall:New {
    name                    = "Gate",
    script                  = "gate.lua",
    footprintX              = 15,
    footprintZ              = 5,
    collisionVolumeScales   = '260 160 50',
    collisionVolumeOffsets  = '0 56 0',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'box',
}

local Electrafi = BaseEffect:New {
    name                = "Electrafi",
    script              = "electrafi.lua",
    customParams = {
        electric_field = true,
    },
}

local Fire = BaseEffect:New {
    name                = "Fire",
    script              = "fire.lua",
    customParams = {
        fire = true,
    },
}

local Plate = BaseWall:New {
    name                = "Plate",
    script              = "plate.lua",
    footprintX          = 8,
    footprintZ          = 8,
}

local Wall1 = BaseWall:New {
    name                    = "Wall1",
    script                  = "wall1.lua",
    footprintX              = 30,
    footprintZ              = 5,
    collisionVolumeScales   = '490 160 50',
    collisionVolumeOffsets  = '0 56 0',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'box',
}

local Wall2 = BaseWall:New {
    name                    = "Wall2",
    script                  = "wall2.lua",
    footprintX              = 15,
    footprintZ              = 5,
    collisionVolumeScales   = '260 160 50',
    collisionVolumeOffsets  = '0 56 0',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'box',
}

local Wall3 = BaseWall:New {
    name                    = "Wall3",
    script                  = "wall3.lua",
    footprintX              = 7,
    footprintZ              = 5,
    collisionVolumeScales   = '140 160 50',
    collisionVolumeOffsets  = '0 56 0',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'box',
}

local Wall4 = BaseWall:New {
    name                    = "Wall4",
    script                  = "wall4.lua",
    footprintX              = 3,
    footprintZ              = 5,
    collisionVolumeScales   = '70 160 50',
    collisionVolumeOffsets  = '0 56 0',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'box',
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
