local CannonBase = Weapon:New {
    weaponType              = "LaserCannon",
    --impactOnly              = true,
    --noSelfDamage            = true,
    impulseFactor           = 1,
    turret                  = true,
    
--     avoidFriendly           = false,
--     collideFriendly         = false,
--     avoidFeature            = false,
--     avoidGround             = false,
--     collideGround           = false,
--     collideNeutral          = false,
}


local GravityBase = Weapon:New {
      areaOfEffect            = 8,
      avoidFriendly           = false,
      burst                   = 9,
      burstrate               = 0.01,
      coreThickness           = 0.5,
      craterBoost             = 0,
      craterMult              = 0,

      damage                  = {
        default = 0.001,
        planes  = 0.001,
        subs    = 5E-05,
      },

      duration                = 0.0333,
      endsmoke                = [[0]],
      explosionGenerator      = [[custom:NONE]],
      impactOnly              = true,
      intensity               = 0.7,
      interceptedByShieldType = 0,
      noSelfDamage            = true,
      predictBoost            = 1,
      proximityPriority       = -15,
      range                   = 460,
      reloadtime              = 0.2,
      renderType              = 4,
      rgbColor                = [[0 0 1]],
      rgbColor2               = [[1 0.5 1]],
      size                    = 2,
      soundTrigger            = true,
      startsmoke              = [[0]],
      thickness               = 4,
      tolerance               = 5000,
      turret                  = true,
      weaponTimer             = 0.1,
      weaponType              = [[LaserCannon]],
      weaponVelocity          = 2200,
}

return {
    CannonBase = CannonBase,
    GravityBase = GravityBase,
}
