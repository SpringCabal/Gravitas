--  Custom Options Definition Table format

--  NOTES:
--  - using an enumerated table lets you specify the options order

--
--  These keywords must be lowercase for LuaParser to read them.
--
--  key:      the string used in the script.txt
--  name:     the displayed name
--  desc:     the description (could be used as a tooltip)
--  type:     the option type
--  def:      the default value
--  min:      minimum value for number options
--  max:      maximum value for number options
--  step:     quantization step, aligned to the def value
--  maxlen:   the maximum string length for string options
--  items:    array of item strings for list options
--  scope:    "all", "player", "team", "allyteam"      <<< not supported yet >>>
--

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
--
--  Example EngineOptions.lua 
--

local options = 
{
 
  {
    key    = "MaxUnits",
    name   = "Max units",
    desc   = "Determines the ceiling of how many units and buildings a player is allowed  to own at the same time (key = 'MaxUnits')",
    type   = "number",
    def    = 1000,
    min    = 1,
    max    = 10000,
    step   = 1,  -- quantization is aligned to the def value
                    -- (step <= 0) means that there is no quantization
  },
  
  {
    key    = "GhostedBuildings",
    name   = "Ghosted buildings",
    desc   = "Once an enemy building will be spotted\n a ghost trail will be placed to memorize location even after the loss of the line of sight (key = 'GhostedBuildings')",
    type   = "bool",
    def    = true,
  },

  {
    key    = "FixedAllies",
    name   = "Fixed ingame alliances",
    desc   = "Disables the possibility of players to dynamically change allies ingame (key = 'FixedAllies')",
    type   = "bool",
    def    = false,
  },

  {
    key    = "LimitSpeed",
    name   = "Speed Restriction",
    desc   = "Limits maximum and minimum speed that the players will be allowed to change to (key = 'LimitSpeed')",
    type   = "section",
  },
  
  {
    key    = "MaxSpeed",
    name   = "Maximum game speed",
    desc   = "Sets the maximum speed that the players will be allowed to change to (key = 'MaxSpeed')",
    type   = "number",
    section= "LimitSpeed",
    def    = 3,
    min    = 0.1,
    max    = 100,
    step   = 0.1,  -- quantization is aligned to the def value
                    -- (step <= 0) means that there is no quantization
  },
  
  {
    key    = "MinSpeed",
    name   = "Minimum game speed",
    desc   = "Sets the minimum speed that the players will be allowed to change to (key = 'MinSpeed')",
    type   = "number",
    section= "LimitSpeed",
    def    = 0.3,
    min    = 0.1,
    max    = 100,
    step   = 0.1,  -- quantization is aligned to the def value
                    -- (step <= 0) means that there is no quantization
  },
  
  {
    key    = "DisableMapDamage",
    name   = "Undeformable map",
    desc   = "Prevents the map shape from being changed by weapons (key = 'DisableMapDamage')",
    type   = "bool",
    def    = true,
  },

}
return options
