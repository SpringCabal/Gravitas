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

function SetBindings()
    local binds = { --real keybinds
        
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
        
        "a attack",
        "s stop",
        "w jump",
        "q onoff",         
    }

    for _,binding in pairs(binds) do
        Spring.SendCommands("bind ".. binding)
    end
end

function widget:Initialize()
    local devMode = (tonumber(Spring.GetModOptions().play_mode) or 0) == 0
    if not devMode then 
        Spring.SendCommands("unbindall") --muahahahaha
    end
    SetBindings()
    
    bindText = { -- keybinds told to player
        "A : gravity gun",
        "Q : swap pull/push",
        "W : jump",
        "S : stop",
    }
    
    mouseText = {
        "Left mouse : fire",
        "Right mouse : move",
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
    
    for _,text in ipairs(bindText) do
        AddLine(text,x,y)
        y = y - h
    end
    for _,text in ipairs(mouseText) do
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