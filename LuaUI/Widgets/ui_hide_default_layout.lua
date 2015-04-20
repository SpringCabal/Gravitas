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
    -- check if scenario editor is run in play mode
    local devMode = (tonumber(Spring.GetModOptions().play_mode) or 0) == 0
    if not devMode then
        Spring.SendCommands("Console 0")
        -- disable wait and patrol in play mode
        -- FIXME: this still doesn't disable shift+w and shift+p
        Spring.SendCommands("unbindkeyset w", "unbindkeyset p")
        Spring.SendCommands({"unbindkeyset e", "bind e jump"})
    end

    -- remove Springs default UI stuff
    Spring.SendCommands("ResBar 0", "ToolTip 0", "Clock 0", "Info 0")
    gl.SlaveMiniMap(true)
    gl.ConfigMiniMap(-1,-1,-1,-1)
end

-- sets status to ready & hide pre-game UI
function widget:GameSetup()
    return true, true
end
