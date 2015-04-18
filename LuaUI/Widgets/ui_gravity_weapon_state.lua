-- WIP
function widget:GetInfo()
	return {
		name    = 'Gravity weapon state',
		desc    = 'Displays gravity weapon state',
		author  = 'gajop, Bluestone, Funkencool',
		date    = 'April, 2015',
		license = 'GNU GPL v2',
        layer = 0,
		enabled = true,
	}
end

local Chili, window

local spGetMyTeamID      = Spring.GetMyTeamID
local myTeamID = spGetMyTeamID()

local onOffState
local onOffStateBtn
local attackCmd
-------------------------------------------

local function resizeUI(vsx,vsy)
    if onOffStateBtn ~= nil then
        onOffStateBtn:SetPos(vsx*0.28, vsy*(1-0.10-0.01), vsx*0.07, vsy*0.10) 
    end
end

local function cmdAction(obj, x, y, button, mods)
	if obj.disabled then return end
    local index = Spring.GetCmdDescIndex(obj.cmdId)
    if (index) then
        local left, right = (button == 1), (button == 3)
        local alt, ctrl, meta, shift = mods.alt, mods.ctrl, mods.meta, mods.shift
        Spring.SetActiveCommand(index, button, left, right, alt, ctrl, meta, shift)
    end
end

local function addOnOffState(cmd)
    local param = cmd.params[cmd.params[1] + 2]
    if param:find("On") then
        param = "\255\255\0\255Push\b"
    else
        param = "\255\0\255\255Pull\b"
    end
    onOffStateBtn = Chili.Button:New{
		caption   = param,
		cmdName   = attackCmd.name,
		tooltip   = cmd.tooltip,
		cmdId     = attackCmd.id,
		cmdAName  = attackCmd.action,
		padding   = {10,0,0,0},
		margin    = {0,0,0,0},
		OnMouseUp = {cmdAction},
		font      = {
			size  = 20,
		},
        parent = Chili.Screen0,
        children = {
            Chili.Label:New {
                caption = "Q to change",
                bottom = 10,
                x = 0,
                font = {
                    size = 12,
                },
            },
        },
		--backgroundColor = black,
	}
    local vsx,vsy = Spring.GetViewGeometry()
    resizeUI(vsx,vsy)
end

local function parseCmds()
	-- Parses through each cmd and gives it its own button
    onOffState = nil
    if onOffStateBtn ~= nil then
        onOffStateBtn:Dispose()
        onOffStateBtn = nil
    end
	for _, cmd in pairs(Spring.GetActiveCmdDescs()) do
		if cmd.action == "onoff" then
            onOffState = cmd
        elseif cmd.action == "attack" then
            attackCmd = cmd
        end
	end
    if onOffState ~= nil then
        addOnOffState(onOffState)
    end
end

function widget:ViewResize(vsx,vsy)
	resizeUI(vsx,vsy)
end

function widget:Initialize()
	Chili = WG.Chili
    if Spring.GetGameFrame()>0 then
        parseCmds()
    end
    
    local vsx,vsy = Spring.GetViewGeometry()
    resizeUI(vsx,vsy)
end

function widget:CommandsChanged()
	updateRequired = true
end

function widget:Update()
    if updateRequired then
        parseCmds()
        updateRequired = false
    end
end