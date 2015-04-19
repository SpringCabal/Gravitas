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
}

GRAVITY_NEG = GravityBase:New{
      name                    = [[Attractive Gravity]],
      soundStart              = [[weaponpull.ogg]],
      customParams            = {
        impulse = [[-125]],
      },
}

GRAVITY_POS = GravityBase:New{
      name                    = [[Repulsive Gravity]],
      soundStart              = [[weaponpush.ogg]],
      customParams            = {
        impulse = [[125]],
      },

}



return lowerkeys({
    GRAVITY_NEG = GRAVITY_NEG,
    GRAVITY_POS = GRAVITY_POS,
    Kannon = Kannon,
    BobGun = BobGun,
    WallesRevenge = WallesRevenge,
})