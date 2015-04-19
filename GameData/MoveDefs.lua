-- Wiki: http://springrts.com/wiki/Movedefs.lua
-- See also; http://springrts.com/wiki/Units-UnitDefs#Tag:movementClass

local moveDefs  =    {
    {
        name            =   "Gravit",
        footprintX      =   2,
        footprintZ      =   2,
        maxWaterDepth   =   10,
        maxSlope        =   36,
        crushStrength   =   5,
        heatmapping     =   false,
        allowTerrainCollisions  = false,
    },
    {
        name            =   "KBOT_Infantry",
        footprintX      =   1,
        maxWaterDepth   =   10,
        maxSlope        =   36,
        crushStrength   =   5,
        heatmapping     =   false,
        allowTerrainCollisions  = false,
    },
    {
        name = "Wall5x17",
        footprintX = 3,
        footprintZ = 20,
        maxWaterDepth = 10,
        maxSlope = 2,
        crushStrength = 55,
        allowTerrainCollisions  = false,
    },
    {
        name = "Default2x2",
        footprintX = 2,
        maxWaterDepth = 10,
        maxSlope = 20,
        crushStrength = 25,
    },
}

return moveDefs