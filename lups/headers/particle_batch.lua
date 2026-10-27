-- Shared instanced world-space batches. Unit/piece/projectile space keeps its
-- display-list path, with a flush before the caller changes the model matrix.
local vertex = VFS.LoadFile('lups/headers/particle_batch.vert')
local normalFragment = [[#version 330 compatibility
uniform sampler2D tex0;
void main() { gl_FragColor = texture2D(tex0, gl_TexCoord[0].st) * gl_Color; }
]]
local distortionFragment = [[#version 330 compatibility
uniform sampler2D noiseMap;
varying vec2 noiseCoord;
varying vec2 radial;
varying float strength;
varying float heat;
void main() {
    vec2 noiseVec = (texture2D(noiseMap, noiseCoord).xy - 0.50) * strength;
    noiseVec *= smoothstep(1.0, 0.0, dot(radial, radial));
    gl_FragColor = vec4(noiseVec, length(noiseVec) * heat, gl_FragCoord.z);
}
]]
-- Colormaps are float32 values. Compare cached values directly instead of
-- formatting every channel into a string for each newly created effect.
local palettes = {}
local function sameChannel(a,b)
    if a==b then return a~=0 or 1/a==1/b end
    if type(a)~='number' or type(b)~='number' or (a~=a and b~=b) then
        return string.format('%.9g',a)==string.format('%.9g',b)
    end
    return false
end
local function palette(colors)
    if not colors then return end
    local count = #colors
    local bucket = palettes[count]
    if bucket then
        for cached in pairs(bucket) do
            local same = #cached==count
            if same then
                for i=1,count do
                    for j=1,4 do
                        if not sameChannel(colors[i][j],cached[i][j]) then same=false;break end
                    end
                    if not same then break end
                end
            end
            if same then return cached end
        end
    else
        bucket = setmetatable({}, {__mode='k'})
        palettes[count] = bucket
    end
    bucket[colors] = true
    return colors
end

local FX,START,COUNT,DEAD,VISIBLE,SEEN,PALETTE = 1,2,3,4,5,6,7
local Batch = {}
Batch.__index = Batch

function Batch.new(distortion)
    if not gl.GetVBO or not gl.GetVAO then return end
    local self = setmetatable({distortion=distortion, groups={}, queued={}, serial=0, paletteData={}, paletteIds={}}, Batch)
    self.shader = gl.CreateShader({vertex=vertex,
        fragment=distortion and distortionFragment or normalFragment,
        uniformInt={tex0=0, noiseMap=0, distortion=distortion and 1 or 0}})
    if not self.shader then
        Spring.Echo('LUPS particle batch shader failed: ' .. gl.GetShaderLog())
        return
    end
    self.colors, self.time = {}, gl.GetUniformLocation(self.shader, 'time')
    self.ageFrame = gl.GetUniformLocation(self.shader, 'ageFrame')
    for i=1,128 do self.colors[i] = gl.GetUniformLocation(self.shader, 'colors[' .. (i-1) .. ']') end
    self.quad = gl.GetVBO(GL.ARRAY_BUFFER, false)
    self.quad:Define(6, {{id=0, name='corner', size=2}})
    self.quad:Upload({0,0,1,0,1,1,0,0,1,1,0,1})
    return self
end

local attributes = {}
for i=1,7 do attributes[i] = {id=i, name='particle'..i, size=4} end

local function upload(group, entry)
    group.instances:Upload(entry[FX].gpuParticles, nil, entry[START])
end

local function allocateBuffers(self, group, capacity)
    if group.vao then group.vao:Delete(); group.instances:Delete() end
    group.capacity = capacity
    group.instances = gl.GetVBO(GL.ARRAY_BUFFER, true)
    group.instances:Define(capacity, attributes)
    group.vao = gl.GetVAO()
    group.vao:AttachVertexBuffer(self.quad)
    group.vao:AttachInstanceBuffer(group.instances)
end

local function setVisible(group, entry, visible)
    if entry[VISIBLE] == visible then return end
    entry[VISIBLE] = visible
    local data = entry[FX].gpuParticles
    for i=20,#data,28 do data[i] = visible and entry[PALETTE] or -1 end
    upload(group, entry)
end

local function paletteOffset(self, colors)
    if not colors then return 0 end
    if self.paletteIds[colors] then return self.paletteIds[colors] end
    local offset = #self.paletteData
    assert(offset + #colors <= 128, 'LUPS batch exceeds 128 palette colors')
    for i=1,#colors do self.paletteData[offset+i] = colors[i] end
    self.paletteIds[colors] = offset
    return offset
end

local function register(self, fx)
    local key = fx.texture .. ':' .. fx.layer
    local group = self.groups[key]
    if not group then
        group = {texture=fx.texture, entries={}, first=1, head=0, capacity=0}
        self.groups[key] = group
    end
    -- Entries expire in allocation order. Holes behind a longer-lived effect
    -- remain clipped by the shader until the ring can reclaim the prefix.
    while group.entries[group.first] and group.entries[group.first][DEAD] do
        group.first = group.first + 1
    end
    if group.first > 1024 and group.first > #group.entries / 2 then
        local entries = {}
        for i=group.first,#group.entries do entries[#entries+1] = group.entries[i] end
        group.entries,group.first = entries,1
    end
    local count = #fx.gpuParticles / 28
    local oldest = group.entries[group.first]
    local start = group.head
    if not oldest then
        group.entries, group.first, group.head = {}, 1, 0
        start = 0
    end
    local tail = oldest and oldest[START] or 0
    local room = false
    if not oldest then room = count <= group.capacity
    elseif start > tail then
        if start + count <= group.capacity then room = true
        elseif count <= tail then start,room = 0,true end
    elseif start < tail then room = start + count <= tail end
    if not room then
        local alive, used = {}, 0
        for i=group.first,#group.entries do
            local entry = group.entries[i]
            if not entry[DEAD] then
                entry[START] = used
                used = used + entry[COUNT]
                alive[#alive+1] = entry
            end
        end
        local capacity = math.max(256,group.capacity)
        while capacity < used + count do capacity = capacity*2 end
        allocateBuffers(self, group, capacity)
        group.entries, group.first, group.head = alive,1,used
        for i=1,#alive do upload(group,alive[i]) end
        start = used
    end
    local offset = paletteOffset(self, fx.gpuPalette)
    local data = fx.gpuParticles
    local birth = self.frame - fx.frame
    for i=1,#data,28 do data[i+3],data[i+19] = birth,offset end
    local entry = {fx,start,count,false,true,self.serial,offset}
    fx.gpuEntry,fx.gpuGroup = entry,group
    group.entries[#group.entries+1] = entry
    group.head = start + count
    upload(group,entry)
    return entry,group
end

function Batch:begin(frame)
    self.serial = self.serial + 1
    self.frame = frame
end

function Batch:flush(legacyShader)
    if #self.queued == 0 then return end
    gl.UseShader(self.shader)
    gl.Uniform(self.ageFrame, self.frame)
    if self.distortion then gl.Uniform(self.time, self.noiseFrame)
    elseif self.colorCount ~= #self.paletteData then
        for i=1,#self.paletteData do gl.Uniform(self.colors[i], unpack(self.paletteData[i])) end
        self.colorCount = #self.paletteData
    end
    for _,group in ipairs(self.queued) do
        local ranges = {}
        local start,finish
        for i=group.first,#group.entries do
            local entry = group.entries[i]
            setVisible(group, entry, not entry[DEAD] and entry[SEEN] == self.serial)
            if not entry[DEAD] then
                if finish and entry[START] < finish then
                    ranges[#ranges+1],ranges[#ranges+2] = start,finish
                    start = nil
                end
                start = start or entry[START]
                finish = entry[START] + entry[COUNT]
            end
        end
        if start then ranges[#ranges+1],ranges[#ranges+2] = start,finish end
        -- Finish visibility uploads before drawing. Reuse their scan to find
        -- the ring wraps, preserving allocation and blending order.
        gl.Texture(0,group.texture)
        for i=1,#ranges,2 do
            group.vao:DrawArrays(GL.TRIANGLES,6,0,ranges[i+1]-ranges[i],ranges[i])
        end
        group.queued = false
    end
    self.queued = {}
    self.serial = self.serial + 1
    gl.UseShader(legacyShader)
end

function Batch.worldspace(fx)
    return not ((fx.unit and fx.unit >= 0 and not fx.worldspace)
        or (fx.projectile and not fx.worldspace))
end

function Batch:queue(fx, legacyShader)
    if not fx.gpuOnly then return false end
    local entry,group = fx.gpuEntry,fx.gpuGroup
    if not entry then entry,group = register(self,fx) end
    if fx.gpuReset then
        for i=4,#fx.gpuParticles,28 do fx.gpuParticles[i] = self.frame-fx.frame end
        upload(group,entry)
    end
    fx.gpuReset = false
    entry[SEEN] = self.serial
    if not group.queued then
        self.queued[#self.queued+1],group.queued = group,true
    end
    return true
end

function Batch.release(fx)
    if not fx.gpuEntry then return end
    fx.gpuEntry[DEAD] = true
    if thisGameFrame and fx.dieGameFrame and thisGameFrame > fx.dieGameFrame then
        -- Every particle has already expired. The age test clips interior
        -- holes; changing their palette would upload invisible data again.
        fx.gpuEntry[VISIBLE] = false
    else
        -- Explicit early removal still needs to hide retained ring slots.
        setVisible(fx.gpuGroup,fx.gpuEntry,false)
    end
    fx.gpuEntry,fx.gpuGroup = nil,nil
end

function Batch:delete()
    for _,group in pairs(self.groups) do group.vao:Delete(); group.instances:Delete() end
    self.quad:Delete()
    gl.DeleteShader(self.shader)
end

-- Called while the original attribute generator runs, consuming no new RNG.
function Batch.prepare(self)
    self.gpuParticles = {}
    self.gpuReset = false
    self.gpuEntry = false
    self.gpuGroup = false
    self.gpuPalette = palette(self.colormap)
end

function Batch.pack(self, life, delay, x,y,z, dx,dy,dz, sizeStart,sizeEnd,
        rotStart,rotEnd, animX,animY)
    local data = self.gpuParticles
    local at = #data
    data[at+1],data[at+2],data[at+3],data[at+4] = x,y,z,0
    data[at+5],data[at+6],data[at+7],data[at+8] = dx,dy,dz,life
    data[at+9],data[at+10],data[at+11],data[at+12] = self.force[1],self.force[2],self.force[3],delay
    data[at+13],data[at+14],data[at+15],data[at+16] = sizeStart,sizeEnd,rotStart or 0,rotEnd or 0
    data[at+17],data[at+18],data[at+19],data[at+20] = self.speedExp,self.sizeExp,self.rotExp or 1,0
    data[at+21],data[at+22] = animX or 0,animY or 0
    data[at+23],data[at+24] = self.strength or 0,(self.heat or 0)/(self.strength or 1)
    data[at+25],data[at+26],data[at+27],data[at+28] = self.scale or 0,self.ncolors or 0,self.forceExp,self.animSpeed or 1
end

return Batch
