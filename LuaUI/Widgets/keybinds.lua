function widget:GetInfo()
  return {
    name      = "Keybinds",
    desc      = "",
    author    = "Bluestone",
    date      = "in the future",
    license   = "GPL-v2",
    layer     = -10000,
    enabled   = true,
  }
end

local bindText, mouseText
local Chili, screen0
local children = {}
local x,y,h

local purple = "\255\255\10\255"
local white = "\255\255\255\255"

function SetBindings()
    local binds = { --real keybinds
        "Any+esc quitforce",
        "Any+pause  pause",
        "Any+p  pause",
        "Alt+b  debug",
        "Alt+v  debugcolvol",
    }
    for _,binding in pairs(binds) do
        Spring.SendCommands("bind ".. binding)
    end
end

function widget:Initialize()
    local devMode = (tonumber(Spring.GetModOptions().play_mode) or 0) == 0
    if not devMode then
        Spring.SendCommands("unbindall") --muahahahaha
        Spring.SendCommands("unbindkeyset enter chat") --because because.
    else
        --Spring.SendCommands("unbindkeyset w", "unbindkeyset p", "unbindkeyset q", "unbindkeyset a", "unbindkeyset s") --why p?
    end
    SetBindings()


    mouseText = {
        purple.."Left Mouse Button : "..white.." Push",
        purple.."Right Mouse Button : "..white.." Pull",
        purple.."Middle Mouse Button : "..white.." Jump",
    }

    bindText={
        purple.."WASD or arrow keys : "..white.." Move",
        purple.."E or Space : "..white.." Jump",
    }


    if (not WG.Chili) then
        return
    end
    Chili = WG.Chili
    screen0 = Chili.Screen0

    MakeBindingText()
end


function MakeBindingText()
    if (not WG.Chili) then
        return
    end

    for _,child in pairs(children) do
        screen0:RemoveChild(child)
    end


    h = 20
    y = h*(#bindText + #mouseText)
    x = 10

    for _,text in ipairs(mouseText) do
        AddLine(text,x,y)
        y = y - h
    end
    for _,text in ipairs(bindText) do
        AddLine(text,x,y)
        y = y - h
    end
end

function  AddLine(text,x,y,h)
    children[#children+1] = Chili.Label:New{
        x = x,
        bottom = y,
        parent = screen0,
        caption = text,
    }
end
