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
local LOG_LEVEL = LOG.NOTICE

if (gadgetHandler:IsSyncedCode()) then

local plates = {}
function gadget:UnitCreated(unitID, unitDefID, unitTeam, builderID)
    if UnitDefs[unitDefID].name == "plate" then
        plates[unitID] = { pressed = false }
    end
    -- EXAMPLE:
    if UnitDefs[unitDefID].customParams.gate then
        for plateID, _ in pairs(plates) do
            SimpleLink(plateID, unitID)
        end
    end
end

function gadget:UnitDestroyed(unitID, unitDefID, ...)
    if UnitDefs[unitDefID].name == "plate" then
        plates[unitID] = nil
    end
end

function SimpleLink(plateID, gateID)
    if plates[plateID] == nil then
        Spring.Log(LOG_SECTION, "error", "No such plate with ID: ", plateID)
        return
    end
    Spring.Log(LOG_SECTION, LOG_LEVEL, "Linking plate " .. tostring(plateID) .. " with gate: " .. tostring(gateID))
    plates[plateID].gateID = gateID
end

local updateRate = 13
local plateRadius = 100
function gadget:GameFrame()
    if Spring.GetGameFrame() % updateRate == 0 then
        for plateID, plate in pairs(plates) do
            if plate.gateID then
                local x, _, z = Spring.GetUnitPosition(plateID)
                local units = Spring.GetUnitsInCylinder(x, z, 40)
                local hasToggler = false
                for _, unitID in pairs(units) do
                    if UnitDefs[Spring.GetUnitDefID(unitID)].customParams.plate_toggler then
                        hasToggler = true
                        break
                    end
                end
                local _, _, _, _, active = Spring.GetUnitStates(plate.gateID)
                if active ~= hasToggler then
                    if hasToggler then
                        Spring.GiveOrderToUnit(plate.gateID, CMD.ONOFF, { 1 }, {})
                    else
                        Spring.GiveOrderToUnit(plate.gateID, CMD.ONOFF, { 0 }, {})
                    end
                end
            else
                Spring.Log(LOG_SECTION, LOG_LEVEL, "Plate has no gate: " .. tostring(plateID))
            end
        end
    end
end

GG.Plate = {
    SimpleLink = SimpleLink,
}

end