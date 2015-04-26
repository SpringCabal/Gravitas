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
		width=80;
		right=0;
		height=40;
		caption="Exit",
		OnClick = {
			function() Spring.SendCommands("quit","quitforce") end
		};
		parent = window_endgame;
	}
    
   
    Button:New{
		y=40+20;
		width=80;
		right=0;
		height=40;
		caption="Restart",
		OnClick = {
			function() Spring.Restart("", 
[[
    [GAME]
{
	MapName=Gravitas Enterprise;
	GameMode=0;
	GameType=Scenario Editor Gravitas git;


	NumTeams=2;
	NumUsers=2;

	HostIP=127.0.0.1;
	HostPort=8452;
	IsHost=1;
	NumPlayers=1;
    GameStartDelay=0;

	StartMetal=1000;
	StartEnergy=1000;

	StartposType=3;
	LimitDGun=0;
	DiminishingMMs=0;
	GhostedBuildings=1;
	MyPlayerNum=1;
	MyPlayerName=0;
	NumRestrictions=0;
	MaxSpeed=20;
	MinSpeed=0.1;
	[MODOPTIONS]
	{
        play_mode = 1;
        deathmode = neverend;
        has_scenario_file = 0;
        project_dir = projects/Gravity-Enterprise;
	}

    [TEAM1]
    {
        AllyTeam=1;
        Side=;
        RGBColor=0.7843137383461 0 0;

        TeamLeader=1;
        Handicap=0;
        StartPosX=0;
        StartPosZ=0;
    }
    [AI1]
    {
		Name=1: NullAI;
		ShortName=NullAI;
		Team=1;
		IsFromDemo=0;
		Host=1;
		[Options] {}
    }
    [TEAM0]
    {
        AllyTeam=0;
        Side=;
        RGBColor=0.35294118523598 0.35294118523598 1;

        TeamLeader=1;
        Handicap=0;
        StartPosX=0;
        StartPosZ=0;
    }
    [PLAYER1]
    {
        Name=0;
        Spectator=0;
        Team=0;
    }
    [ALLYTEAM0]
    {
        NumAllies=0;
    }
    [ALLYTEAM1]
    {
        NumAllies=0;
    }

}
]]
            ) 
            end
		};
		parent = window_endgame;
	}

    -- allows work with scened
    local devMode = (tonumber(Spring.GetModOptions().play_mode) or 0) == 0
    if devMode then
        Button:New{
            y=100;
            width='80';
            right=0;
            height=40;
            caption="Close me (dev mode)",
            OnClick = {
                function() window_endgame:Dispose() end
            };
            parent = window_endgame;
        }
    end
    
	screen0:AddChild(window_endgame)

end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
--callins
function widget:Initialize()
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

function widget:GameOver(winningAllyTeams)
    local myAllyTeamID = Spring.GetMyAllyTeamID()
    for _, winningAllyTeamID in pairs(winningAllyTeams) do
        if myAllyTeamID == winningAllyTeamID then
            Spring.SendCommands("endgraph 0")
            SetupControls()
            caption:SetCaption("You win!");
            caption.font.color={0,1,0,1};
            ShowEndGameWindow()
            return
        end
    end
    Spring.SendCommands("endgraph 0")
    SetupControls()
    ShowEndGameWindow()
end


