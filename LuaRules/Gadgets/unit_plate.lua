function gadget:GetInfo()
	return {
		name = "Plates and gates",
		desc = "Would like you a plate?",
		author = "gajop",
		date = "April 2015",
		license = "GNU GPL, v2 or later",
		layer = 1,
		enabled = true
	}
end

local LOG_SECTION = "plate-gate"
local LOG_LEVEL = LOG.DEBUG

if (gadgetHandler:IsSyncedCode()) then

local plates = {}
local bitmaskLinks = {}
local linkChecksEnabled = false
local reportedError = false
local UPDATE_RATE = 5
local PLATE_ACTIVATION_RANGE = 40

function gadget:UnitCreated(unitID, unitDefID, unitTeam, builderID)
    if UnitDefs[unitDefID].name == "plate" then
        plates[unitID] = { pressed = false }
        Spring.SetUnitCollisionVolumeData(unitID, 
            0, 0, 0, 
            0, 0, 0, 
            0, 0, 0);
        Spring.SetUnitBlocking(unitID, false, false, false, false, false, false, false)
    end
--     -- EXAMPLE:
--     if UnitDefs[unitDefID].customParams.gate then
--         for plateID, _ in pairs(plates) do
--             SimpleLink(plateID, unitID)
--         end
--     end
end

function gadget:UnitDestroyed(unitID, unitDefID, ...)
    if plates[unitID] then
        plates[unitID] = nil
    end
    if bitmaskLinks[unitID] then
        bitmaskLinks[unitID] = nil
    end
end

function SimpleLink(plateID, gateID)
    if plates[plateID] == nil then
        Spring.Log(LOG_SECTION, "error", "SimpleLink: No such plate with ID: ", plateID)
        return
    end
    if not UnitDefs[Spring.GetUnitDefID(gateID)].customParams.gate then
        Spring.Log(LOG_SECTION, "error", "SimpleLink: Trying to link plate with non-gate: ", gateID)
        return
    end
    Spring.Log(LOG_SECTION, LOG_LEVEL, "Linking plate " .. tostring(plateID) .. " with gate: " .. tostring(gateID))
    plates[plateID].gateID = gateID
end

function BitmaskLink(plateMask, gateID)
    for _, plateObj in pairs(plateMask) do
        local plateID = plateObj[1]
        if plates[plateID] == nil then
            Spring.Log(LOG_SECTION, "error", "BitwiseLink: No such plate with ID: ", plateID)
            return
        end
    end
    if not UnitDefs[Spring.GetUnitDefID(gateID)].customParams.gate then
        Spring.Log(LOG_SECTION, "error", "BitwiseLink: Trying to link plate with non-gate: ", gateID)
        return
    end
    for _, plateObj in pairs(plateMask) do
        local plateID = plateObj[1]
        plates[plateID].bitmaskLink = true
    end
    bitmaskLinks[gateID] = plateMask
end

function EnableLinkChecks()
    linkChecksEnabled = true
    reportedError = false
end

function DisableLinkChecks()
    linkChecksEnabled = false
end

local function GetUnitState(unitID)
    if plates[unitID] then 
        return plates[unitID].state
    end
    local _, _, _, _, active = Spring.GetUnitStates(unitID)
    return active
end

local function SetUnitState(unitID, state)
    if plates[unitID] then 
        plates[unitID].state = state
    end
    local _, _, _, _, active = GetUnitState(unitID)
    if active ~= state then
        if state then
            Spring.GiveOrderToUnit(unitID, CMD.ONOFF, { 1 }, {})
        else
            Spring.GiveOrderToUnit(unitID, CMD.ONOFF, { 0 }, {})
        end
    end
end

function gadget:GameFrame()
    if not linkChecksEnabled then
        return
    end

    if Spring.GetGameFrame() % UPDATE_RATE == 0 then
        -- check if plates are toggled or not
        for plateID, plate in pairs(plates) do
            if plate.gateID or plate.bitmaskLink then
                local x, y, z = Spring.GetUnitPosition(plateID)
                local units = Spring.GetUnitsInCylinder(x, z, PLATE_ACTIVATION_RANGE)
                local newState = false
                for _, unitID in pairs(units) do
                    if UnitDefs[Spring.GetUnitDefID(unitID)].customParams.plate_toggler then
                        newState = true
                        break
                    end
                end
                SetUnitState(plateID, newState)
            elseif not reportedError then
                Spring.Log(LOG_SECTION, LOG_LEVEL, "Plate has no gate: " .. tostring(plateID))
            end
        end
        reportedError = true

        -- issue simple links
        for plateID, plate in pairs(plates) do
            if plate.gateID then
                SetUnitState(plate.gateID, GetUnitState(plateID))
            end
        end
        -- issue bitmask links
        for gateID, bitmaskLink in pairs(bitmaskLinks) do
            local totalState = 1
            for _, plateObj in pairs(bitmaskLink) do
                if plateObj[2] ~= GetUnitState(plateObj[1]) then
                    totalState = false
                    break
                end
            end
            SetUnitState(gateID, totalState)
        end
    end
end

GG.Plate = {
    BitmaskLink = BitmaskLink,
    SimpleLink = SimpleLink,
    EnableLinkChecks = EnableLinkChecks,
    DisableLinkChecks = DisableLinkChecks,
}

end