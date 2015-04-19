local CannonBase = Weapon:New {
    weaponType              = "LaserCannon",
    --impactOnly              = true,
    --noSelfDamage            = true,
    impulseFactor           = 1,
    turret                  = true,
    onlyTargetCategory      = "INFANTRY",
    
--     avoidFriendly           = false,
--     collideFriendly         = false,
--     avoidFeature            = false,
--     avoidGround             = false,
--     collideGround           = false,
--     collideNeutral          = false,
}

local MissileBase = Weapon:New {
    weaponType              = "MissileLauncher",
    startVelocity           = 10,
    acceleration            = 2,
    tracks                  = true,
    turnRate                = 10,
    flightTime              = 15,
}


local GravityBase = Weapon:New {
      weaponType              = [[BeamLaser]],

      areaOfEffect            = 8,
      craterBoost             = 0,
      craterMult              = 0,
      duration                = 0.1,
      impactOnly              = true,
      noSelfDamage            = true,
      range                   = 420,
      reloadtime              = 0.1,
      tolerance               = 32000, 
      turret                  = true,

      avoidFriendly           = false,
      collideGround           = true,

      damage                  = {
        default = 0.001,
      },
      onlyTargetCategory      = "INFANTRY",

      beamTime                = 0.1,
      coreThickness           = 0.5,
      intensity               = 0.7,
      renderType              = 4,
      rgbColor                = [[0 0 1]],
      rgbColor2               = [[1 0.5 1]],
      size                    = 2,

      startsmoke              = [[0]],
      endsmoke                = [[0]],
      explosionGenerator      = [[custom:NONE]],
      soundTrigger            = true,
      thickness               = 4,
}

return {
    CannonBase = CannonBase,
    GravityBase = GravityBase,
    MissileBase = MissileBase,
}