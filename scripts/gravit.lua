
function script.HitByWeapon(x, z, weaponDefID, damage)
end

local armright = piece('armright');
local armleft = piece('armleft');
local footleft = piece('footleft');
local footright = piece('footright');
local forearmleft = piece('forearmleft');
local forearmright = piece('forearmright');
local handleft = piece('handleft');
local handright = piece('handright');
local head = piece('head');
local kneeleft = piece('kneeleft');
local kneeright = piece('kneeright');
local muzzleleft = piece('muzzleleft');
local muzzleright = piece('muzzleright');
local pelvis = piece('pelvis');
local thighleft = piece('thighleft');
local thighright = piece('thighright');
local torso = piece('torso');

local scriptEnv = {
 armright = armright,
 armleft = armleft,
 footleft = footleft,
 footright = footright,
 forearmleft = forearmleft,
 forearmright = forearmright,
 handleft = handleft,
 handright = handright,
 head = head,
 kneeleft = kneeleft,
 kneeright = kneeright,
 muzzleleft = muzzleleft,
 muzzleright = muzzleright,
 pelvis = pelvis,
 thighleft = thighleft,
 thighright = thighright,
 torso = torso,
 x_axis = x_axis,
 y_axis = y_axis,
 z_axis = z_axis,
}

local SIG_WALK = "walk";

for k,v in pairs(scriptEnv) do
	Spring.Echo(k..' = '..tostring(v));
end

local Animations = {};

Animations['stop'] = VFS.Include("scripts/animations/gravit_stop.lua", scriptEnv);
Animations['idle'] = VFS.Include("scripts/animations/gravit_idle.lua", scriptEnv);
Animations['walk'] = VFS.Include("scripts/animations/gravit_walk.lua", scriptEnv);

function constructSkeleton(unit, piece, offset)
    if (offset == nil) then
        offset = {0,0,0};
    end

    local bones = {};
    local info = Spring.GetUnitPieceInfo(unit,piece);

    for i=1,3 do
        info.offset[i] = offset[i]+info.offset[i];
    end 

    bones[piece] = info.offset;
    local map = Spring.GetUnitPieceMap(unit);
    local children = info.children;

    if (children) then
        for i, childName in pairs(children) do
            local childId = map[childName];
            local childBones = constructSkeleton(unit, childId, info.offset);
            for cid, cinfo in pairs(childBones) do
                bones[cid] = cinfo;
            end
        end
    end        
    return bones;
end

local animCmd = {['turn']=Turn,['move']=Move};
function PlayAnimation(animname, snap)
    local anim = Animations[animname];
    for i = 1, #anim do
        local commands = anim[i].commands;
        for j = 1,#commands do
			-- hack to make first keyframe snap 
            local cmd = commands[j];
            local speed = cmd.s;
            if i == 1 and snap then speed = 0; end
            
            animCmd[cmd.c](cmd.p,cmd.a,cmd.t,speed);
        end
        if(i < #anim) then
            local t = anim[i+1]['time'] - anim[i]['time'];
            Sleep(t*33); -- sleep works on milliseconds
        end
    end
end

function script.Create()
    local map = Spring.GetUnitPieceMap(unitID);
    local offsets = constructSkeleton(unitID,map.Scene, {0,0,0});
    
    for a,anim in pairs(Animations) do
        for i,keyframe in pairs(anim) do
            local commands = keyframe.commands;
            for k,command in pairs(commands) do
                -- commands are described in (c)ommand,(p)iece,(a)xis,(t)arget,(s)peed format
                -- the t attribute needs to be adjusted for move commands from blender's absolute values
                if (command.c == "move") then
                    local adjusted =  command.t - (offsets[command.p][command.a]);
                    Animations[a][i]['commands'][k].t = command.t - (offsets[command.p][command.a]);
                end
            end
        end
    end
    
    PlayAnimation('stop');
end
              
function script.Killed(recentDamage, _)
    return 1
end


----aiming & fire weapon
function script.AimFromWeapon1() 
    return head 
end

function script.QueryWeapon1() 
    return muzzleleft
end

function script.AimWeapon1(Heading, pitch)
    --aiming animation: instantly turn the gun towards the enemy
    return Spring.GetUnitStates(unitID).active
end

function script.FireWeapon1()
    return true
end

function script.AimFromWeapon2() 
    return head 
end

function script.QueryWeapon2() 
    return muzzleright
end

function script.AimWeapon2(Heading, pitch)
    --aiming animation: instantly turn the gun towards the enemy
    return not Spring.GetUnitStates(unitID).active
end

function script.FireWeapon2()
    return true
end

local function Idle()
	Signal(SIG_WALK);
	SetSignalMask(SIG_WALK);
	PlayAnimation('stop',true);
	while true do
		PlayAnimation("idle", false);
	end
end

local function Walk()
	Signal(SIG_WALK)
	SetSignalMask(SIG_WALK)
	PlayAnimation("walk", true);
	while true do
		PlayAnimation("walk", false);
	end
end

function script.StartMoving()
	Signal(SIG_WALK);
	StartThread(Walk);
end

function script.StopMoving()
	Signal(SIG_WALK);
	StartThread(Idle);
end

function script.Activate()
    return 1
end

function script.Deactivate()
    return 0
end

function script.QueryBuildInfo()
    return head 
end

function script.QueryNanoPiece()
    return head
end
