function widget:GetInfo()
  return {
    name      = "Hides the default interface",
    desc      = "",
    author    = "gajop",
    date      = "in the future",
    license   = "GPL-v2",
    layer     = 1001,
    enabled   = true,
  }
end

function widget:Initialize()
    Spring.SendCommands("Console 1")

    -- remove Springs default UI stuff
    Spring.SendCommands("ResBar 0", "ToolTip 0", "Clock 0", "Info 0")
    gl.SlaveMiniMap(true)
    gl.ConfigMiniMap(-1,-1,-1,-1)
end

-- sets status to ready & hide pre-game UI
function widget:GameSetup()
    return true, true
end
