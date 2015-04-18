-- Wiki: http://springrts.com/wiki/Movedefs.lua
-- See also; http://springrts.com/wiki/Units-UnitDefs#Tag:movementClass

local moveDefs 	=	 {
	{
		name					=	"KBOT_Infantry",
		footprintX		=	1,
		maxWaterDepth	=	10,
		maxSlope			=	36,
		crushStrength	=	0,
		heatmapping		=	false,
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