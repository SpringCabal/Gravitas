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
    reloadtime              = 5.0,
    range                   = 110,
--     soundHit                = [[explosion.ogg]],
--     soundStart              = [[laser.ogg]],
    impulseFactor           = 0.2,
    rgbColor                = {150, 0, 190},
    rgbColor                = {200, 0, 140},
    weaponVelocity          = 1030,
    duration                = 0.15,
    thickness               = 6,
    coreThickNess           = 6,
    impactOnly              = true,
    maxDamage               = 15000,
    damage                  = {
        default = 700.1,
    },
}

local WallesRevenge = MissileBase:New {
    name                    = "WallesRevenge",
    areaOfEffect            = 75,
    cegTag                  = [[missiletrailred]],

    damage                  = {
        default = 280,
    },

    craterMult              = 0,
    fireStarter             = 70,
    flightTime              = 3.5,
    impulseBoost            = 0,
    impulseFactor           = 0.4,
    interceptedByShieldType = 2,
    --model                   = [[wep_m_hailstorm.s3o]],
    --texture2                = [[darksmoketrail]],
    noSelfDamage            = true,
    predictBoost            = 1,
    range                   = 530,
    reloadtime              = 10,
    smokeTrail              = true,
    --       soundHit                = [[explosion/ex_med4]],
    --       soundHitVolume          = 8,
    --       soundStart              = [[weapon/missile/missile2_fire_bass]],
    --       soundStartVolume        = 7,
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