--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

function widget:GetInfo()
  return {
    name      = "Chili EndGame Window",
    desc      = "Derived from v0.005 Chili EndGame Window by CarRepairer",
    author    = "Anarchid",
    date      = "April 2015",
    license   = "GNU GPL, v2 or later",
    layer     = 0,
    enabled   = true,
  }
end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
local spSendCommands			= Spring.SendCommands

local echo = Spring.Echo

local caption

local Chili
local Image
local Button
local Checkbox
local Window
local Panel
local ScrollPanel
local StackPanel
local Label
local screen0
local color2incolor
local incolor2color

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

local window_endgame

local function ShowEndGameWindow()
	screen0:AddChild(window_endgame)
end

local function SetupControls()
	window_endgame = Window:New{  
		name = "GameOver",
		caption = "Game Over",
		x = '20%',
		y = '20%',
		width  = '60%',
		height = '60%',
		padding = {8, 8, 8, 8};
		--autosize   = true;
		--parent = screen0,
		draggable = true,
		resizable = true,
		minWidth=500;
		minHeight=400;
	}
	
 	caption = Chili.Label:New{
 		x = '20%',
 		y = '40%',
 		width = 100,
 		parent = window_endgame,
 		caption = "You died.",
 		fontsize = 80,
 		textColor = {1,0,0,1},
 	}
	
	Button:New{
		y=0;
		width='80';
		right=0;
		height=B_HEIGHT;
		caption="Exit",
		OnClick = {
			function() Spring.SendCommands("quit","quitforce") end
		};
		parent = window_endgame;
	}
	
	screen0:AddChild(window_endgame)

end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
--callins
function widget:Initialize()
    -- allows work with scened
    local devMode = (tonumber(Spring.GetModOptions().play_mode) or 0) == 0
    if devMode then
        widgetHandler:RemoveWidget(widget)
        return
    end
    
	if (not WG.Chili) then
		widgetHandler:RemoveWidget()
		return
	end
	

	Chili = WG.Chili
	Image = Chili.Image
	Button = Chili.Button
	Checkbox = Chili.Checkbox
	Window = Chili.Window
	Panel = Chili.Panel
	ScrollPanel = Chili.ScrollPanel
	StackPanel = Chili.StackPanel
	Label = Chili.Label
	screen0 = Chili.Screen0
	color2incolor = Chili.color2incolor
	incolor2color = Chili.incolor2color

end

function widget:UnitDestroyed(unitID, unitDefID, unitTeam, attackerID, attackerDefID, attackerTeam)
    if UnitDefs[unitDefID].name == "gravit" then
        Spring.SendCommands("endgraph 0")
        SetupControls()
        ShowEndGameWindow()
    elseif UnitDefs[unitDefID].name == "cometopapa" then
        SetupControls()
		caption:SetCaption("You win!");
		caption.font.color={0,1,0,1};
        ShowEndGameWindow()
    end
end

function widget:Shutdown()
end


