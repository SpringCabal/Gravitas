function widget:GetInfo()
  return {
    name      = "Keybinds",
    desc      = "",
    author    = "Bluestone",
    date      = "in the future",
    license   = "GPL-v2",
    layer     = -math.huge,
    enabled   = true,
  }
end

function widget:Initialize()
    local devMode = (tonumber(Spring.GetModOptions().play_mode) or 0) == 0
    if devMode then return end

    Spring.SendCommands("unbindall") --muahahahaha

    local binds = {
        
        "esc  quitmenu",
        
        "Any+enter  chat",
        "Any+enter  edit_return",
        "Any+escape  edit_escape",
        "Any+tab  edit_complete",
        "Any+backspace  edit_backspace",
        "Any+delete  edit_delete",
        "Any+left  edit_prev_char",
        "Any+right  edit_next_char",
        "Ctrl+left  edit_prev_word",
        "Ctrl+right  edit_next_word",
        
        "Any+pause  pause",
        -- space pause (widget)
        
        "Alt+b  debug",
        "Alt+v  debugcolvol",
        
        "Any+up  moveforward",
        "Any+down  moveback",
        "Any+right  moveright",
        "Any+left  moveleft",
        "Any+pageup  moveup",
        "Any+pagedown  movedown",
        "Any+shift  movefast",
        "Any+ctrl  moveslow",
        
        "a  attack",
        "s  stop",
        -- e jump (gadget)
        "q onoff",
           
    }

    for _,binding in pairs(binds) do
        Spring.Echo(binding)
        Spring.SendCommands("bind ".. binding)
    end
end
