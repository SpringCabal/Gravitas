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

local function SetBindings()
    Spring.SendCommands({
        "bind Any+esc reloadforce",
        "bind Any+p pause",
        "bind Alt+b debug",
        "bind Alt+v debugcolvol",
    })
end

function widget:Initialize()
    SetBindings()


    mouseText = {
        purple.."Left Mouse Button : "..white.." Push",
        purple.."Right Mouse Button : "..white.." Pull",
        purple.."Middle Mouse Button : "..white.." Jump",
    }

    bindText={
        purple.."WASD or arrow keys : "..white.." Move",
        purple.."E or Space : "..white.." Jump",
        purple.."Ctrl+Q : "..white.." Quit",
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
