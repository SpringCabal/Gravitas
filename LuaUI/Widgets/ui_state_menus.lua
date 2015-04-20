function widget:GetInfo()
	return {
		name      = 'State & Command Menus',
		desc      = 'Interface for issuing (non-build) commands',
		author    = 'Bluestone, Funkencool, gajop',
		date      = 'April 2015',
		license   = 'GNU GPL v2',
		layer     = 0,
		enabled   = true,
		handler   = true,
	}
end
--------------

-- Config --
local imageDir = 'luaui/images/'

local ignoreCMDs = {
	["TimeWait"] 	 = '',
	["DeathWait"]    = '',
	["SquadWait"]	 = '',
	["GatherWait"]	 = '',
}

local topStates = {
    [1] = "movestate",
    [2] = "firestate",
    [3] = "repeat",
}

local white = {1,1,1,1}
local black = {0,0,0,1}
local green = {0,1,0,1}
local yellow = {0.5,0.5,0,1}
local orange = {1,0.5,0,1}
local red = {1,0,0,1}
local grey = {0.2,0.2,0.2,1}

local paramColors = {
	['Hold fire']    = red,
	['Return fire']  = orange,
	['Fire at will'] = green,
	['Hold pos']     = red,
	['Maneuver']     = orange,
	['Roam']         = green,
	['Repeat off']   = red,
	['Repeat on']    = green,
	['Active']       = green,
	['Passive']      = red,
	[' Fly ']        = green,
	['Land']         = red,
	[' Off ']        = red,
	[' On ']         = green,
	['UnCloaked']    = red,
	['Cloaked']      = green,
	['LandAt 0']     = red,
	['LandAt 30']    = orange,
	['LandAt 50']    = yellow,
	['LandAt 80']    = green,
	['UpgMex off']   = red,
	['UpgMex on']    = green,
	['Low traj']     = red,
	['High traj']    = green,
}

------------


-- Chili vars --
local Chili
local screen0, buildMenu, stateMenu
local stateArray = {}
local unit = {}
----------------

-- Spring Functions --
local spGetTimer          = Spring.GetTimer
local spDiffTimers        = Spring.DiffTimers
local spGetActiveCmdDesc  = Spring.GetActiveCmdDesc
local spGetActiveCmdDescs = Spring.GetActiveCmdDescs
local spGetActiveCommand  = Spring.GetActiveCommand
local spGetCmdDescIndex   = Spring.GetCmdDescIndex
local spGetSelectedUnits  = Spring.GetSelectedUnits
local spSendCommands      = Spring.SendCommands
local spSetActiveCommand  = Spring.SetActiveCommand

-- Local vars --
local updateRequired = true
local sUnits = {}
local oldTimer = spGetTimer()
local r,g,b = Spring.GetTeamColor(Spring.GetMyTeamID())
local teamColor = {r,g,b,0.8}
local gameStarted = (Spring.GetGameFrame()>0)
----------------

local function getInline(r,g,b)
    if not r then r = 1 end
    if not g then g = 1 end
    if not b then b = 1 end
	if type(r) == 'table' then --fixme /0
		return string.char(255, (r[1]*255), (r[2]*255), (r[3]*255))
	else
		return string.char(255, (r*255), (g*255), (b*255))
	end
end

---------------------------------------------------------------

local function resizeUI(vsx,vsy)
	local ordH = vsy * 0.05
	local ordY = vsy - ordH - vsy*0.01
	local winY = vsy * 0.2
	local winH = vsy * 0.5
    
	stateMenu:SetPos(vsx*0.01,vsy*0.01,200,winY)
end

function widget:ViewResize(vsx,vsy)
	resizeUI(vsx,vsy)
end

---------------------------------------------------------------

local function cmdAction(obj, x, y, button, mods)
	if obj.disabled then return end
	if gameStarted then
		local index = spGetCmdDescIndex(obj.cmdId)
		if (index) then
			local left, right = (button == 1), (button == 3)
			local alt, ctrl, meta, shift = mods.alt, mods.ctrl, mods.meta, mods.shift
			spSetActiveCommand(index, button, left, right, alt, ctrl, meta, shift)
		end  
    end
end

local function addState(cmd)
	local param = cmd.params[cmd.params[1] + 2]
	stateMenu:AddChild(Chili.Button:New{
		caption   = param,
		cmdName   = cmd.name,
		tooltip   = cmd.tooltip,
		cmdId     = cmd.id,
		cmdAName  = cmd.action,
		padding   = {0,0,0,0},
		margin    = {0,0,0,0},
		OnMouseUp = {cmdAction},
		font      = {
			color = paramColors[param] or white,
			size  = 16,
		},
		--backgroundColor = black,
	})
end

local function addDummyState(cmd)
	stateMenu:AddChild(Chili.Button:New{
		caption   = cmd.action,
		--tooltip   = cmd.tooltip, 
		padding   = {0,0,0,0},
		margin    = {0,0,0,0},
		OnMouseUp = {},
		font      = {
			color = grey,
			size  = 16,
		},
		--backgroundColor = black,
	})
end

local function AddInSequence(cmds, t, func, dummyFunc)
    for _,k in ipairs(t) do
        if cmds[k] then
            -- add top cmd
            func(cmds[k])
            cmds[k] = nil
        else
            -- add dummy top cmd
            dummy_cmd = {action=k}
            dummyFunc(dummy_cmd)
        end
    end
    
    -- add the rest
    for _,cmd in pairs(cmds) do
        func(cmd)
    end    
end 

local function parseCmds()
	local cmdList = spGetActiveCmdDescs()
    local states = {}
    
	-- Parses through each cmd and gives it its own button
	for i = 1, #cmdList do
		local cmd = cmdList[i]
		if cmd.name ~= '' and not ignoreCMDs[cmd.name] then
			if #cmd.params > 1 then
				states[cmd.action] = cmd
			end
		end
	end
    
    -- Add the states in the wanted order, from L->R
    if #cmdList>0 then
        AddInSequence(states, topStates, addState, addDummyState)
    end
end

--------------------------------------------------------------

-- Loads/reloads the icon panels for commands
local function loadPanels()
	local newUnit = false
	local units = spGetSelectedUnits()
	if #units == #sUnits then
		for i = 1, #units do
			if units[i] ~= sUnits[i] then
				newUnit = true
				break
			end
		end
	else
		newUnit = true
	end

	stateMenu:ClearChildren()

	stateArray = {}

	if newUnit then
		sUnits = units
	end

	parseCmds()
end

function widget:Initialize()
	if Spring.GetGameFrame()>0 then gameStarted = true end

	if (not WG.Chili) then
		widgetHandler:RemoveWidget()
		return
	end

	Chili = WG.Chili
	screen0 = Chili.Screen0
	
	stateMenu = Chili.Grid:New{
		parent  = screen0,
		columns = 2,
		rows    = 8,
        orientation = 'vertical',
		padding = {0,0,0,0},
	}
    local stateMenuInfo = Chili.Label:New{
        parent = screen0,
        caption = "State menu: only visible in dev mode",
    }
    
    -- hide states in play mode
    local devMode = (tonumber(Spring.GetModOptions().play_mode) or 0) == 0
    if not devMode then
        stateMenu:Hide()
        stateMenuInfo:Hide()
    end
    
    local vsx,vsy = Spring.GetViewGeometry()
	resizeUI(vsx,vsy)
end

function widget:CommandsChanged()
	updateRequired = true
end

function widget:Update()
	if updateRequired then
		local r,g,b = Spring.GetTeamColor(Spring.GetMyTeamID())
		teamColor = {r,g,b,0.8}
		updateRequired = false
		loadPanels()
	end
end

function widget:GameStart()
	gameStarted = true
	updateRequired = true
end

function widget:UnitCreated(unitID, unitDefID, unitTeam, builderID)
    local HOLD_FIRE = 0
    if UnitDefs[unitDefID].customParams.player and unitTeam == Spring.GetMyTeamID() then
        Spring.GiveOrderToUnit(unitID, CMD.INSERT, 
            { 0, CMD.FIRE_STATE, 0, HOLD_FIRE},{"alt"})
    end
end