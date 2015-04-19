--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
if not gadgetHandler:IsSyncedCode() then
	return
end
--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

function gadget:GetInfo()
  return {
    name      = "Impulse",
    desc      = "",
    author    = "[Fx]Bluestone, based on original gadget by Google Frog",
    date      = "April 20152012",
    license   = "GNU GPL, v3 or later",
    layer     = 0,
    enabled   = true
  }
end

--local GRAVITY = Game.gravity
local GRAVITY_BASELINE = 120
local UNSTICK_CONSTANT = 1 -- hack

local spAddUnitImpulse = Spring.AddUnitImpulse
local spGetUnitDefID = Spring.GetUnitDefID
local spGetUnitPosition = Spring.GetUnitPosition
local spGetUnitStates = Spring.GetUnitStates

local abs = math.abs

-------------------------------------------------------------------------------------
-- populate impulseWeaponID
local impulseWeaponID = {}
for i, wd in pairs(WeaponDefs) do
	local impulse
	for k, v in pairs(wd.customParams) do
		if k == "impulse" then
			impulse = tonumber(wd.customParams.impulse)
			break
		end
	end
	--Spring.Echo(wd.name, wd.id)
	if impulse then
		--Spring.Echo(wd.name, wd.id)
		impulseWeaponID[wd.id] = impulse
	end
end

-- populate mass
local mass = {}
for i=1,#UnitDefs do
	local ud = UnitDefs[i]
	mass[i] = ud.mass
end

-- Euclidean distance
local function distance(x1,y1,z1,x2,y2,z2)
	return math.sqrt((x1-x2)^2 + (y1-y2)^2 + (z1-z2)^2)
end

-------------------------------------------------------------------------------------

local thereIsStuffToDo = false
local unitByID = {count = 0, data = {}}
local unit = {}

-- Accumulate impulses (per gameframe)
local function AddImpulse(unitID, unitDefID, x, y, z, magnitude)
	local _, _, inbuild = Spring.GetUnitIsStunned(unitID)
	if inbuild then
		return
	end
	
	local dis = math.sqrt(x^2 + y^2 + z^2)	
	local myMass = mass[unitDefID]
	local mag = magnitude*GRAVITY_BASELINE/dis*(0.0032)/myMass
	
	x,y,z = x*mag, y*mag, z*mag
    y = y + abs(magnitude)/(20*myMass)
	
	if not unit[unitID] then
		unit[unitID] = {
			moveType = moveType,
			useDummy = useDummy,
			pushOffGround = pushOffGround,
			x = x, y = y, z = z
		}
		unitByID.count = unitByID.count + 1
		unitByID.data[unitByID.count] = unitID
	else
		unit[unitID].x = unit[unitID].x + x
		unit[unitID].y = unit[unitID].y + y
		unit[unitID].z = unit[unitID].z + z
	end
	thereIsStuffToDo = true
end

function gadget:UnitPreDamaged(unitID, unitDefID, unitTeam, damage, paralyzer, weaponDefID, projectileID, attackerID, attackerDefID, attackerTeam)
	if impulseWeaponID[weaponDefID] and Spring.ValidUnitID(attackerID) then
		local ux, uy, uz = spGetUnitPosition(unitID, true)
		local ax, ay, az = spGetUnitPosition(attackerID, true)
		
		local x,y,z = (ux-ax), (uy-ay), (uz-az)
		local magnitude = impulseWeaponID[weaponDefID]
		
		AddImpulse(unitID, unitDefID, x, y, z, magnitude) 		
	end
	return damage,1.0
end

-- Action impulses
local function ActionImpulses()
	if thereIsStuffToDo then
		for i = 1, unitByID.count do
			local unitID = unitByID.data[i]
			local data = unit[unitID]
            --Spring.Echo(unitID, data.x, data.y, data.x, UNSTICK_CONSTANT)
            
			--spAddUnitImpulse(unitID, UNSTICK_CONSTANT,0,0) --dummy impulse (applying impulse>1 make unit less sticky to map surface)
			spAddUnitImpulse(unitID, data.x, data.y, data.z)
			--spAddUnitImpulse(unitID, -UNSTICK_CONSTANT,0,0) --remove dummy impulse
		end
		unitByID = {count = 0, data = {}}
		unit = {}
		thereIsStuffToDo = false
	end
end

function gadget:GameFrame(f)
	ActionImpulses()
end

