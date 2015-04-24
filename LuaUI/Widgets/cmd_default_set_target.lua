function widget:GetInfo()
	return {
	name	= "Set target & attack by default",
	desc	= "Changes default click from attack to set target",
	author	= "BD, Bluestone",
	date	= "-",
	license	= "WTFPL",
	layer	= -math.huge,
	enabled	= 0,
	}
end

local rebindKeys = false

local CMD_UNIT_SET_TARGET = 34923

local GetMouseState = Spring.GetMouseState
local TraceScreenRay = Spring.TraceScreenRay
local IsUnitAllied = Spring.IsUnitAllied
local GetSelectedUnitsCounts = Spring.GetSelectedUnitsCounts
local GetActionHotKeys = Spring.GetActionHotKeys
local SendCommmands = Spring.SendCommands

local hotKeys = {}

function hasSetTarget(unitDefID)
	local ud = UnitDefs[unitDefID]
	return ud and ( ( ud.canMove and ud.speed > 0 and not ud.canFly and ud.canAttack and ud.maxWeaponRange and ud.maxWeaponRange > 0 ) or ud.isFactory )
end

function widget:DefaultCommand()
	local targettype,data = TraceScreenRay(GetMouseState())
	if targettype ~= "unit" or IsUnitAllied(data) then
		return
	end
	for unitDefID in pairs(GetSelectedUnitsCounts()) do
		if hasSetTarget(unitDefID) then
			return CMD_UNIT_SET_TARGET
		end
	end
end
