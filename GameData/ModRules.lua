--Wiki: http://springrts.com/wiki/Modrules.lua

local modRules = {
  movement = {
    allowPushingEnemyUnits   = true,
    allowCrushingAlliedUnits = false,
    allowUnitCollisionDamage = true,
    allowUnitCollisionOverlap = false,
    allowGroundUnitGravity = false,
  },
 system = {
        pathFinderSystem = 1,
	  },
}

return modRules
