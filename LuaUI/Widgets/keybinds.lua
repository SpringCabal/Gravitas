function widget:GetInfo()
  return {
    name      = "Adds keybinds",
    desc      = "",
    author    = "Bluestone",
    date      = "in the future",
    license   = "GPL-v2",
    layer     = 1001,
    enabled   = true,
  }
end

function widget:Initialize()
    Spring.SendCommands("bind q onoff")    
end
