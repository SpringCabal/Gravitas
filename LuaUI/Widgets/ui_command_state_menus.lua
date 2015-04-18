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
	selfd 	     = '',
	loadonto 	   = '',
	timewait 	   = '',
	deathwait    = '',
	squadwait	   = '',
	gatherwait	 = '',
    
    attack = "",
    move = "",
    fight = "",
    patrol = "",
    stop = "",
    repair = "",
    guard = "",
    wait = "",
}

local orderColors = {
	wait        = {0.8, 0.8, 0.8 ,1.0},
	attack      = {1.0, 0.0, 0.0, 1.0},
	fight       = {1.0, 0.3, 0.0, 1.0},
	areaattack  = {0.8, 0.0, 0.0, 1.0},
	manualfire  = {0.8, 0.2, 0.0, 1.0},
	move        = {0.2, 1.0, 0.0, 1.0},
	reclaim     = {0.2, 0.6, 0.0, 1.0},
	patrol      = {0.4, 0.4, 1.0, 1.0},
	guard       = {0.0, 0.0, 1.0, 1.0},
	repair      = {0.2, 0.2, 0.8, 1.0},
	loadunits   = {1.0, 1.0, 1.0, 1.0},
	unloadunits = {1.0, 1.0, 1.0, 1.0},
	stockpile   = {1.0, 1.0, 1.0, 1.0},
	upgrademex  = {0.6, 0.6, 0.6, 1.0},
	capture     = {0.6, 0.0, 0.8, 1.0},
	resurrect   = {0.0, 0.0, 1.0, 1.0},
    restore     = {0.5, 1.0, 0.2, 1.0},
    stop        = {0.4, 0.0, 0.0, 1.0},
    leechlife   = {0.7, 0.1, 0.7, 1.0},
}

local topOrders = {
--     [1] = "move",
--     [2] = "fight",
--     [3] = "attack",
--     [4] = "patrol",
--     [5] = "stop",
--     [6] = "repair",
--     [7] = "guard",
--     [8] = "wait",
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

local Hotkey = {
    ["attack"] = "A",
    ["guard"] = "G",
    ["fight"] = "F",
    ["patrol"] = "P",
    ["reclaim"] = "E",
    ["loadonto"] = "L",
    ["loadunits"] = "L",
    ["unloadunit"] = "U",
    ["unloadunits"] = "U",
    ["stop"] = "S",
    ["wait"] = "W",
    ["repair"] = "R",
    ["manualfire"] = "D",
    ["cloak"] = "K",
    ["move"] = "M",
    ["resurrect"] = "O",
}
------------


-- Chili vars --
local Chili
local panH, panW, winW, winH, winX, winB, tabH, minMapH, minMapW
local screen0, buildMenu, stateMenu, orderMenu, orderBG, menuTabs 
local orderArray = {}
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
    
	orderMenu:SetPos(vsx*0.01,ordY,ordH*21,ordH)
	orderBG:SetPos(vsx*0.01,ordY,ordH*#orderMenu.children,ordH)
	stateMenu:SetPos(vsx*0.01,vsy*0.01,200,winY)
end

function widget:ViewResize(vsx,vsy)
	resizeUI(vsx,vsy)
end

---------------------------------------------------------------

local function HotkeyString(action)
    if Hotkey[action] then
        return " " .. "[" .. Hotkey[action] .. "]"
    else
        return ""
    end
end

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

local function addOrder(cmd)
	local button = Chili.Button:New{
		caption   = '',
		cmdName   = cmd.name,
		tooltip   = cmd.tooltip .. getInline(orderColors[cmd.action]) .. HotkeyString(cmd.action),
		cmdId     = cmd.id,
		cmdAName  = cmd.action,
		padding   = {0,0,0,0},
		margin    = {0,0,0,0},
		OnMouseUp = {cmdAction},
		Children  = {
			Chili.Image:New{
				parent  = button,
				x       = 5,
				bottom  = 5,
				y       = 5,
				right   = 5,
				color   = orderColors[cmd.action] or {1,1,1,1},
				file    = imageDir..'Commands/'..cmd.action..'.png',
			}
		}
	}

    if cmd.id==CMD.STOCKPILE then
        for _,uID in ipairs(sUnits) do -- we just pick the first unit that can stockpile
            local n,q = Spring.GetUnitStockpile(uID)
            if n and q then
                local stockpile_q = Chili.Label:New{right=0,bottom=0,caption=n.."/"..q, font={size=14,shadow=false,outline=true,autooutlinecolor=true,outlineWidth=4,outlineWeight=6}}
                button.children[1]:AddChild(stockpile_q)
                break
            end
        end
    end

	orderMenu:AddChild(button)
	orderBG:Resize(orderMenu.height*#orderMenu.children,orderMenu.height)
end

local function addDummyOrder(cmd)
	local button = Chili.Button:New{
		caption   = '',
		--tooltip   = cmd.tooltip .. getInline(orderColors[cmd.action]) .. HotkeyString(cmd.action),
		padding   = {0,0,0,0},
		margin    = {0,0,0,0},
		OnMouseUp = {},
		Children  = {
			Chili.Image:New{
				parent  = button,
				x       = 5,
				bottom  = 5,
				y       = 5,
				right   = 5,
				color   = grey,
				file    = imageDir..'Commands/'..cmd.action..'.png',
			}
		}
	}

	orderMenu:AddChild(button)
	orderBG:Resize(orderMenu.height*#orderMenu.children,orderMenu.height)
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
    local orders = {}
    local states = {}
    
	-- Parses through each cmd and gives it its own button
	for i = 1, #cmdList do
		local cmd = cmdList[i]
		if cmd.name ~= '' and not (ignoreCMDs[cmd.name] or ignoreCMDs[cmd.action]) then
			if #cmd.params > 1 then
				states[cmd.action] = cmd
			elseif cmd.id > 0 then
				orderMenu.active = true
				orders[cmd.action] = cmd
			end
		end
	end
    
    -- Include stop command, if needed
    if orderMenu.active then
        local stop_cmd = {name="Stop", action='stop', id=CMD.STOP, tooltip="Clears the command queue"}
        orders[stop_cmd.action] = stop_cmd
    end
    
    -- Add the orders/states in the wanted order, from L->R
    if #cmdList>0 then
        AddInSequence(orders, topOrders, addOrder, addDummyOrder)
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

	orderMenu:ClearChildren()
	stateMenu:ClearChildren()

	orderArray = {}
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
	
	orderMenu = Chili.Grid:New{
		parent  = screen0,
		active  = false,
		columns = 21,
		rows    = 1,
		padding = {0,0,0,0},
	}
	
	orderBG = Chili.Panel:New{
		parent = screen0,
	}

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
		orderMenu.active = false -- if order cmd is found during parse this will become true

		loadPanels()

		if not orderMenu.active and orderBG.visible then
			orderBG:Hide()
		elseif orderMenu.active and orderBG.hidden then
			orderBG:Show()
		end		
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