-- WIP
function widget:GetInfo()
	return {
		name    = 'Quit Button',
		desc    = '',
		author  = 'Bluestone',
		date    = 'April, 2015',
		license = 'Horses',
        layer = 0,
		enabled = true,
	}
end

local Chili, Screen0
local quitButton

function resizeUI(vsx,vsy)
    if quitButton ~= nil then
        quitButton:SetPos(vsx*0.52, vsy*(1-0.05-0.01), vsx*0.05, vsy*0.05) 
    end
end

function widget:ViewResize(vsx,vsy)
	resizeUI(vsx,vsy)
end

function widget:Initialize()
	Chili = WG.Chili
    Screen0 = Chili.Screen0
    
    quitButton = Chili.Button:New{
		caption   = "Quit",
		padding   = {0,0,0,0},
		margin    = {0,0,0,0},
		OnClick = {function() Spring.SendCommands("quitforce")end },
        font = {
            size = 14,
        },
        parent = Chili.Screen0,
		--backgroundColor = black,
	}
    
    local vsx,vsy = Spring.GetViewGeometry()
    resizeUI(vsx,vsy)
end
