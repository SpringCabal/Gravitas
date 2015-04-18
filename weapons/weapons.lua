-- local Claw = MeleeBase:New {
-- 	name				= "Claw",
-- 	reloadtime              = 1.0,
--     range                   = 40,
-- 	damage					= {
-- 		default = 20.1,
-- 	},
-- }
-- 
-- local Staff = MeleeBase:New {
-- 	name				    = "Staff",
-- 	reloadtime              = 0.8,
--     range                   = 60,
-- 	damage					= {
-- 		default = 50.1,
-- 	},
-- }

GRAVITY_NEG = GravityBase:New{
      name                    = [[Attractive Gravity]],
	  customParams            = {
	    impulse = [[-125]],
	  },

}

GRAVITY_POS = GravityBase:New{
      name                    = [[Repulsive Gravity]],
	  customParams            = {
	    impulse = [[125]],
	  },

}



return lowerkeys({
	GRAVITY_NEG = GRAVITY_NEG,
	GRAVITY_POS = GRAVITY_POS,
})