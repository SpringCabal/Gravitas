function gadget:GetInfo()
	return {
		name = "Walls",
		desc = "Walls for justice",
		author = "gajop",
		date = "April 2015",
		license = "GNU GPL, v2 or later",
		layer = 1,
		enabled = true
	}
end

local LOG_SECTION = "wall"
local LOG_LEVEL = LOG.DEBUG

if (gadgetHandler:IsSyncedCode()) then

    local testWall
function gadget:UnitCreated(unitID, unitDefID, unitTeam, builderID)
    -- boom
    Spring.Log(LOG_SECTION, LOG_LEVEL, "Is Wall Building?: ", UnitDefNames["wall1"].isBuilding)
    if UnitDefs[unitDefID].customParams.wall then
        testWall = unitID
        Spring.Log(LOG_SECTION, LOG_LEVEL, "Setting wall stuff: " .. tostring(unitID))
        
    	Spring.SetUnitAlwaysVisible(unitID, true);
-- 		Spring.SetUnitCollisionVolumeData(unitID, 0, 0, 0,
-- 											0, 0, 0,
-- 											0, 0, 0 
-- 		);
-- 		Spring.SetUnitPosition(unitID,512,0,512);
		Spring.SetUnitNeutral(unitID, true)
		Spring.MoveCtrl.Enable(unitID)
--         Spring.MoveCtrl.SetNoBlocking(unitID, true)
		Spring.MoveCtrl.SetGravity(unitID,0)
-- 		Spring.MoveCtrl.SetPhysics(unitID,
-- 		   512,-150,512,
-- 		   0,0,0,
-- 		   0,0,0
-- 		);
-- 		
-- 		
-- 		Spring.SetUnitMidAndAimPos ( unitID, 
-- 		   0, 100, 0, 
-- 		   0, 100, 0,
-- 		   true
-- 		);
    end
end

-- local angle = 0
-- local updateRate = 10
-- function gadget:GameFrame()
--     if Spring.GetGameFrame() % updateRate ~= 0 then
--         return
--     end
--     if testWall then
--         local x = math.sin(math.rad(angle))
--         local z = math.cos(math.rad(angle))
--         Spring.SetUnitBlocking(testWall, false)
--         Spring.SetUnitDirection(testWall, x, 0, z)
--         Spring.SetUnitBlocking(testWall, true)
--         angle = angle + 1
--     end
-- end

end