-- WIP
function widget:GetInfo()
	return {
		name    = 'Cursor tooltip',
		desc    = 'Provides a tooltip whilst hovering the mouse',
		author  = 'Funkencool',
		date    = '2013',
		license = 'GNU GPL v2',
		layer   = 0,
		enabled = true,
	}
end

local Chili, screen, tipWindow, tip
local mousePosX, mousePosY
local oldTime
local tipType = false
local tooltip = ''
local ID

local spGetGameFrame            = Spring.GetGameFrame
local spGetTimer                = Spring.GetTimer
local spDiffTimers              = Spring.DiffTimers
local spTraceScreenRay          = Spring.TraceScreenRay
local spGetMouseState           = Spring.GetMouseState
local spGetUnitTooltip          = Spring.GetUnitTooltip
local spGetUnitResources        = Spring.GetUnitResources
local spGetFeatureResources     = Spring.GetFeatureResources
local spGetFeatureDefID         = Spring.GetFeatureDefID
local screenWidth, screenHeight = Spring.GetWindowGeometry()
-----------------------------------
function firstToUpper(str)
    -- make the first char of a string into upperCase
    return (str:gsub("^%l", string.upper))
end
-----------------------------------
local function initWindow()
	tipWindow = Chili.Panel:New{
		parent    = screen,
		width     = 75,
		height    = 75,
		minHeight = 1,
		padding   = {5,4,4,4},
	}
	tip = Chili.TextBox:New{
		parent = tipWindow, 
		x      = 0,
		y      = 0,
		right  = 0, 
		bottom = 0,
		margin = {0,0,0,0},
        font = {            
            outline          = true,
            autoOutlineColor = true,
            outlineWidth     = 3,
            outlineWeight    = 4,
        }
	}
	
	oldTime = spGetTimer()
	tipWindow:Hide()
end

function widget:ViewResize(vsx, vsy)
	screenWidth = vsx
	screenHeight = vsy
end

-----------------------------------
local function formatresource(description, res)
	color = ""
	if res < 0 then color = '\255\255\127\0' end
	if res > 0 then color = '\255\127\255\0' end
	
	if math.abs(res) > 20 then -- no decimals for small numbers
		res = string.format("%d", res)
		else
		res = string.format("%.1f",res)
	end
	return color .. description .. res
end
-----------------------------------
local function getUnitTooltip(uID)
	local tooltip = spGetUnitTooltip(uID)
	if tooltip==nil then
		tooltip=""
	end
	local metalMake, metalUse, energyMake, energyUse = spGetUnitResources(uID)
	
	local metal = ((metalMake or 0) - (metalUse or 0))
	local energy = ((energyMake or 0) - (energyUse or 0))
	
	tooltip = tooltip..'\n'..formatresource("Metal: ", metal)..'/s\b\n' .. formatresource("Energy: ", energy)..'/s'
	return tooltip
end
-----------------------------------
local function getFeatureTooltip(fID)
	local rMetal, mMetal, rEnergy, mEnergy, reclaimLeft = spGetFeatureResources(fID)
    local fDID = spGetFeatureDefID(fID)
    local fName = FeatureDefs[fDID].tooltip
	local tooltip = "Metal: "..rMetal..'\n'.."Energy: "..rEnergy
    if fName then tooltip = firstToUpper(fName) .. '\n' .. tooltip end
	return tooltip
end
-----------------------------------
local prevTipType, prevID
local function getTooltip()

	-- This gives chili absolute priority
	--  otherwise TraceSreenRay() would ignore the fact ChiliUI is underneath the mouse
	if screen.currentTooltip then
		tipType = 'chili'
		return screen.currentTooltip
	else
		tipType, ID = spTraceScreenRay(spGetMouseState())

        local gameFrame = spGetGameFrame()
		if tipType == prevTipType and ID==prevID and not (tipType=='unit' and gameFrame<=prevGameFrame+3) then --need to update because units build costs change
			return false
		else
			prevTipType = tipType; prevID = ID; prevGameFrame = gameFrame
		end

		if tipType == 'unit'        then
			return getUnitTooltip(ID)
		elseif tipType == 'feature' then 
			return getFeatureTooltip(ID)
		else
			tipType = false
			return ''
		end
	end
end
-----------------------------------
local function setTooltipPos(text)
	local text         = text or tip.text
	local x,y          = spGetMouseState()
	local _,_,numLines = tip.font:GetTextHeight(text)
	local height       = numLines * 14 + 8
	local width        = tip.font:GetTextWidth(text) + 10
	
	x = x + 20
	y = screenHeight - y -- Spring y is from the bottom, chili is from the top

	-- Making sure the tooltip is within the boundaries of the screen	
	y = (y > screenHeight * .75) and (y - height) or (y + 20)
	x = (x + width > screenWidth) and (screenWidth - width) or x

	tipWindow:SetPos(x, y, width, height)
   
	if tipWindow.hidden then tipWindow:Show() end
	tipWindow:BringToFront()
end
-----------------------------------
function widget:Update()
	local curTime = spGetTimer()
	local text = getTooltip()
	
	if text and (tip.text ~= text) then
		tip:SetText(text)
		oldTime = spGetTimer()
		if not tipType and tipWindow.visible then 
			tipWindow:Hide()
		end
	elseif tipType and (tipWindow.visible or spDiffTimers(curTime, oldTime) > 1 or tipType == 'chili') then
		setTooltipPos(text)
	end
end
-----------------------------------
function widget:Initialize()
	if not WG.Chili then return end
	Chili = WG.Chili
	screen = Chili.Screen0
	initWindow()
end

function widget:Shutdown()
	tipWindow:Dispose()
end

