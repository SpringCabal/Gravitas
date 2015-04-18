--related thread: http://springrts.com/phpbb/viewtopic.php?f=13&t=26732&start=22
function widget:GetInfo()
  return {
    name      = "External Grid",
    desc      = "Draws a VR grid around map",
    author    = "knorke, tweaked by KR",
    date      = "Sep 2011",
    license   = "PD",
    layer     = 0,
    enabled   = true,
  }
end

-- TODO: make res and range settable in options

local DspLst=nil
local localAllyID = Spring.GetLocalAllyTeamID ()
--local updateFrequency = 120   -- unused
local gridTex = "LuaUI/Images/vr_grid.png"

---magical speedups---
local math = math
local random = math.random
local spGetGroundHeight = Spring.GetGroundHeight
local glVertex = gl.Vertex
local glTexCoord = gl.TexCoord
local glColor = gl.Color
local glCreateList = gl.CreateList
local glTexRect = gl.TexRect
local spTraceScreenRay = Spring.TraceScreenRay
----------------------

local heights = {}
local island = false
local vsx, vsy
local delayFrame
local initialized = false

--[[
local maxHillSize = 800/res
local maxPlateauSize = math.floor(maxHillSize*0.6)
local maxHeight = 300
local featureChance = 0.01
local noFeatureRange = 0
]]--

options_path = 'Settings/View/Map/Configure VR Grid'
options_order = {"drawForIslands","res","range"}
options = {
        drawForIslands = {
                name = "Draw for islands",
                type = 'bool',
                value = Spring.GetConfigInt("ReflectiveWater", 0) ~= 4, --TODO (e.g. asteroids)
                desc = "Draws grid for islands",                
        },      
        res = {
                name = "Resolution (32-512)",
                advanced = true,
                type = 'number',
                min = 64, 
                max = 512, 
                step = 16,
                value = 128,
                desc = 'Sets resolution (lower = more detail)',
                OnChange = function(self)
                        gl.DeleteList(DspLst)
                        widget:Initialize()
                end, 
        },
        range = {
                name = "Range (1024-8192)",
                advanced = true,
                type = 'number',
                min = 1024, 
                max = 8192, 
                step = 256,
                value = 512,
                desc = 'How far outside the map to draw',
                OnChange = function(self)
                        gl.DeleteList(DspLst)
                        widget:Initialize()
                end, 
        },              
}

local function DistanceFromMapEdge(x,z)
    local dx, dz
    if x < 0 then
        dx = -x
    elseif x > Game.mapSizeX then
        dx =  x - Game.mapSizeX
    else
        dx = 0
    end
    if z < 0 then
        dz = -z
    elseif z > Game.mapSizeZ then
        dz = z - Game.mapSizeZ        
    else
        dz = 0
    end
    return math.max(dx,dz)
end

local function Decay(x)
    -- smooth at 0,1, o at 0, 1 at 1
    if x < 0.5 then
        return x^2
    else
        return 1-(1-x)^2 
    end
end

local function GetGroundHeight(x, z)
    local h = heights[x] and heights[x][z] 
    if h then 
        return h --already computed
    end
    
    local px, pz
    if x < 0 then
        px = - x
    elseif x > Game.mapSizeX then
        px = 2*Game.mapSizeX - x
    else
        px = x
    end
    if z < 0 then
        pz = - z
    elseif z > Game.mapSizeZ then
        pz = 2*Game.mapSizeZ - z
    else
        pz = z
    end
    
    local inMap = (x>0) and (x<Game.mapSizeX) and (z>0) and (z<Game.mapSizeZ)
    local h =  Spring.GetGroundHeight(px, pz)
    local d = DistanceFromMapEdge(x,z) 
    local dmax = options.range.value
    local f = 1-(dmax-d)/dmax
    
    return h * (1-f*f) 
end

local function InitGroundHeights()
        local res = options.res.value or 128
        local range = (options.range.value or 8192)/res
        local TileMaxX = Game.mapSizeX/res +1
        local TileMaxZ = Game.mapSizeZ/res +1
        
        for x = (-range)*res,Game.mapSizeX+range*res, res do
                heights[x] = {}
                for z = (-range)*res,Game.mapSizeZ+range*res, res do
                        heights[x][z] = GetGroundHeight(x,z)       -- 20, 0
                end
        end          
end

local function IsIsland()
        local sampleDist = 640
        for i=1,Game.mapSizeX,sampleDist do
                -- top edge
                if GetGroundHeight(i, 0) > 0 then
                        return false
                end
                -- bottom edge
                if GetGroundHeight(i, Game.mapSizeZ) > 0 then
                        return false
                end
        end
        for i=1,Game.mapSizeZ,sampleDist do
                -- left edge
                if GetGroundHeight(0, i) > 0 then
                        return false
                end
                -- right edge
                if GetGroundHeight(Game.mapSizeX, i) > 0 then
                        return false
                end     
        end
        return true
end

--[[
function widget:GameFrame(n)
        if n % updateFrequency == 0 then
                Spring.Echo("ping")
                DspList = nil
        end
end
]]--

local function TilesVerticesOutside()
        local res = options.res.value or 128
        local range = (options.range.value or 8192)/res
        local TileMaxX = Game.mapSizeX/res +1
        local TileMaxZ = Game.mapSizeZ/res +1   
        for x=-range,TileMaxX+range,1 do
                for z=-range,TileMaxZ+range,1 do
                        if (x > 0 and z > 0 and x < TileMaxX and z < TileMaxZ) then 
                        else
                                glTexCoord(0,0)
                                glVertex(res*(x-1), GetGroundHeight(res*(x-1),res*z), res*z)
                                glTexCoord(0,1)
                                glVertex(res*x, GetGroundHeight(res*x,res*z), res*z)
                                glTexCoord(1,1)                         
                                glVertex(res*x, GetGroundHeight(res*x,res*(z-1)), res*(z-1))
                                glTexCoord(1,0)
                                glVertex(res*(x-1), GetGroundHeight(res*(x-1),res*(z-1)), res*(z-1))
                        end
                end
        end
end

local function DrawTiles()
        gl.PushAttrib(GL.ALL_ATTRIB_BITS)
        gl.DepthTest(true)
        gl.DepthMask(true)
        gl.Texture(gridTex)
        gl.BeginEnd(GL.QUADS,TilesVerticesOutside)
        gl.Texture(false)
        gl.DepthMask(false)
        gl.DepthTest(false)
        glColor(1,1,1,1)
        gl.PopAttrib()
end
function widget:ViewResize()
	vsx, vsy = gl.GetViewSizes()
end

local function CheckVRGridVisible() --pointless?
	local at, p = spTraceScreenRay(0,0,true,false,false)
	if at == nil then
		return true
	else
		at, p = spTraceScreenRay(0,vsy-1,true,false,false)
		if at == nil then
			return true
		else
			at, p = spTraceScreenRay(vsx-1,0,true,false,false)
			if at == nil then
				return true
			else
				at, p = spTraceScreenRay(vsx-1,vsy-1,true,false,false)
				if at == nil then
					return true
				end
			end
		end
	end
	return false
end

function widget:DrawWorld()
    if not initialized then return end
        if ((not island) or options.drawForIslands.value )and CheckVRGridVisible() then
                gl.CallList(DspLst)-- Or maybe you want to keep it cached but not draw it everytime.
                -- Maybe you want Spring.SetDrawGround(false) somewhere
        end     
end

function widget:GameFrame()
    if delayFrame ~= nil and delayFrame == Spring.GetGameFrame() then
        vsx, vsy = gl.GetViewSizes()
        island = IsIsland()
        InitGroundHeights()
        DspLst = glCreateList(DrawTiles)
        Spring.SendCommands('mapborder 0')
        initialized = true
    end
end

function widget:Initialize()
    -- delay
    delayFrame = Spring.GetGameFrame() + 20
end

function widget:Shutdown()
    if not initialized then return end
    gl.DeleteList(DspList)
    Spring.SendCommands('mapborder 1')
end