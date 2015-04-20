local Rocket = MissileBase:New {
    name                    = "Rocket",
    reloadtime              = 2.0,
    range                   = 400,
    soundHit                = [[explosion.ogg]],
    soundStart              = [[laser.ogg]],
    weaponVelocity          = 130,
    damage                  = {
        default = 200.1,
    },
}


local Kannon = CannonBase:New {
    name                    = "Kannon",
    reloadtime              = 2.0,
    range                   = 400,
    soundHit                = [[explosion.ogg]],
    soundStart              = [[laser.ogg]],
    weaponVelocity          = 130,
    damage                  = {
        default = 200.1,
    },
}

local BobGun = CannonBase:New {
    name                    = "BobGun",
    reloadtime              = 0.1,
    range                   = 260,
--     soundHit                = [[explosion.ogg]],
    soundStart              = [[laser.ogg]],
    impulseFactor           = 0.2,
    rgbColor                = {150, 0, 190},
    rgbColor                = {200, 0, 140},
    weaponVelocity          = 1030,
    duration                = 0.15,
    thickness               = 1,
    coreThickNess           = 1,
    impactOnly              = true,
    maxDamage               = 15000,
    turret                  = false,
    damage                  = {
        default = 7.1,
    },
}

local WallesRevenge = MissileBase:New {
    name                    = "WallesRevenge",
    areaOfEffect            = 75,
    cegTag                  = [[missiletrailred]],

    damage                  = {
        default = 280,
    },

    burst                   = 2,
    burstRate               = 0.8,
    craterMult              = 0,
    fireStarter             = 70,
    flightTime              = 7,
    impulseBoost            = 0,
    impulseFactor           = 0.4,
    interceptedByShieldType = 2,
    model                   = [[rocket.s3o]],
    texture2                = [[rockettex1]],
    noSelfDamage            = true,
    predictBoost            = 1,
    range                   = 530,
    reloadtime              = 10,
    smokeTrail              = true,
    soundHit                = [[explosion.ogg]],
    soundStart              = [[rocket.ogg]],
    soundStartVolume        = 10,
    startVelocity           = 170,
    
    tracks                  = false,
    trajectoryHeight        = 0.6,
    turnrate                = 1000,
    turret                  = false,
    weaponVelocity          = 170,
}

GRAVITY_NEG = GravityBase:New{
      name                    = [[Attractive Gravity]],
      soundStart              = [[weaponpull.ogg]],
      rgbColor                = [[0 0 1]],
      rgbColor2               = [[0.3 0.5 1]],
      customParams            = {
        impulse = [[-100]],
      },
}

GRAVITY_POS = GravityBase:New{
      name                    = [[Repulsive Gravity]],
      soundStart              = [[weaponpush.ogg]],
      rgbColor                = [[0 0 1]],
      rgbColor2               = [[1 0.5 1]],
      customParams            = {
        impulse = [[100]],
      },

}



return lowerkeys({
    GRAVITY_NEG = GRAVITY_NEG,
    GRAVITY_POS = GRAVITY_POS,
    Kannon = Kannon,
    BobGun = BobGun,
    WallesRevenge = WallesRevenge,
})