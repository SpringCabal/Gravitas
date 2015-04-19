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

GRAVITY_NEG = GravityBase:New{
      name                    = [[Attractive Gravity]],
      soundStart              = [[weaponpull.ogg]],
      customParams            = {
        impulse = [[-125]],
      },
      range = GravityBase.range + 100,
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
})