function widget:GetInfo()
  return {
    name      = "Standalone lobby tools",
    desc      = "Hides the default interface",
    author    = "gajop",
    date      = "in the future",
    license   = "GPL-v2",
    layer     = 1001,
    enabled   = true,
  }
end

function widget:Initialize()
    -- TODO: Use this line for release (disables the console)
    --Spring.SendCommands("ResBar 0", "ToolTip 0", "Console 0", "Clock 0", "Info 0")
    
    -- check if scenario editor is run in play mode
    Spring.SendCommands("ResBar 0", "ToolTip 0", "Clock 0", "Info 0")
    Spring.SendCommands("bind q onoff")
    local devMode = (tonumber(Spring.GetModOptions().play_mode) or 0) == 0
    if not devMode then
        Spring.SendCommands("Console 0")
        -- disable wait and patrol in play mode
        -- FIXME: this still doesn't disable shift+w and shift+p
        Spring.SendCommands("unbindkeyset w", "unbindkeyset p")
    end
    gl.SlaveMiniMap(true)
    gl.ConfigMiniMap(-1,-1,-1,-1)
end

-- sets status to ready; avoiding any pre-game UI
function widget:GameSetup()
    return true, true
end
