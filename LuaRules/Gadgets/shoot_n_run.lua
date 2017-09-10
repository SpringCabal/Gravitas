
function gadget:GetInfo()
	return {
		name = "Shoot'n'Run",
		desc = "Turn RTS to HnS/SnR",
		author = "zwzsg",
		date = "10th of November 2009",
		license = "Public Domain",
		layer = 120,
		enabled = true
	}
end

local gravit = UnitDefNames.gravit.id
local hero={gravit}
local isHero= {
	[gravit]=true,
}

local AllowOtherUnitsControl=false


if (gadgetHandler:IsSyncedCode()) then
--SYNCED

	local LastHeroesCheck=0
	local ptu={}-- Player to Unit
	local utp={}-- Unit to Player
	local usd={}-- Unit Self Destruct engaged so discard subsequent self-d orders as they would cancel it
	local ptf={}-- Player to frame he was last given a hero
	local RecMsg={}-- To bufferize Received Messages bewteen each call to GameFrame
	local ScoreBoard={}
	local MovedLast={} -- Just to remember which heroes moved last frame

	--function gadget:Initialize()
	--end

	--function gadget:GameStart()
	--end


	-- Return the list of active, non spectator, players of a team
	-- If second argument is zero, then only return those with no hero, sorted by order of last respawn (oldest first)
	local function GetBetterPlayerList(team,ZeroForHeroLessPlayers)
		PlayerList={}
		for _,p in ipairs(Spring.GetPlayerList(team,true)) do
			local _,active,spectator=Spring.GetPlayerInfo(p)
			if active and not spectator then
				if ZeroForHeroLessPlayers==0 then
					if not ptu[p] then
						local inserted=false
						for i,q in pairs(PlayerList) do
							if (ptf[p] or 0)<(ptf[q] or 0) then
								table.insert(PlayerList,i,p)
								inserted=true
								break
							end
						end
						if not inserted then
							table.insert(PlayerList,p)
						end
					end
				else
					table.insert(PlayerList,p)
				end
			end
		end
		return PlayerList
	end


	local function GiveHero(u,team)
		local team=team or Spring.GetUnitTeam(u)
		local PlayerList=GetBetterPlayerList(team,0)
		if #PlayerList>=1 then
			local p=PlayerList[1]
			ptf[p]=Spring.GetGameFrame()
			ptu[p]=u
			utp[u]=p
			Spring.SendMessage("<PLAYER"..p.."> got "..UnitDefs[Spring.GetUnitDefID(u)].humanName.."!")
			Spring.SetUnitCOBValue(u,3,0)-- 3 is STANDINGFIREORDERS
			if not ScoreBoard[p] then
				ScoreBoard[p]={}
			end
		end
	end


	function gadget:GameFrame(frame)

		-- Make unsynced run that function once per frame
		SendToUnsynced("Shoot_n_Run_Unsynced_GameFrame",frame)
		-- Share the table telling which unit are controllled
		_G.Shoot_n_Run={utp=utp,ptu=ptu,ScoreBoard=ScoreBoard}

		-- Parse the messages from unsynced
		for _,msg in pairs(RecMsg) do
			local u=tonumber(string.match(msg,"U:(%d+)"))
			local dx=tonumber(string.match(msg,"MX:(%-?%d+)"))
			local dz=tonumber(string.match(msg,"MZ:(%-?%d+)"))
			local tx=tonumber(string.match(msg,"TX:(%-?%d+%.?%d*)"))
			local ty=tonumber(string.match(msg,"TY:(%-?%d+%.?%d*)"))
			local tz=tonumber(string.match(msg,"TZ:(%-?%d+%.?%d*)"))
			local tu=tonumber(string.match(msg,"TU:(%d+)"))
			local fa=string.match(msg,"LMB")
			local fb=string.match(msg,"RMB")
			local fc=string.match(msg,"MMB")
			local sd=string.match(msg,"SD")
			local jx=tonumber(string.match(msg,"JX:(%-?%d+%.?%d*)"))
			local jy=tonumber(string.match(msg,"JY:(%-?%d+%.?%d*)"))
			local jz=tonumber(string.match(msg,"JZ:(%-?%d+%.?%d*)"))
			--local speed=math.sqrt((dx or 0)^2+(dz or 0)^2)
			dx=dx or 0
			dz=dz or 0
			if u and Spring.ValidUnitID(u) then
				if tu ~= u then
					Spring.SetUnitTarget(u,tu)
				elseif tx and tz then
					if ty then
						Spring.SetUnitTarget(u,tx,ty,tz)
					else
						local ty=Spring.GetGroundHeight(tx,tz)
						Spring.SetUnitTarget(u,tx,ty,tz)
					end
				end
				if not fa and frame>=Spring.GetUnitWeaponState(u,Game.gameName and 1 or 0,"reloadFrame") then
					Spring.SetUnitWeaponState(u,Game.gameName and 1 or 0,"reloadFrame",frame+2)
				end
				if not fb and frame>=Spring.GetUnitWeaponState(u,Game.gameName and 2 or 1,"reloadFrame") then
					Spring.SetUnitWeaponState(u,Game.gameName and 2 or 1,"reloadFrame",frame+2)
				end
				if sd and not usd[u] then
					Spring.GiveOrderToUnit(u,CMD.SELFD,{},{})
					usd[u]=frame
				end
				if frame%3==0 then
					if dx~=0 or dz~=0 then
						local x,_,z=Spring.GetUnitPosition(u)
						local y=Spring.GetGroundHeight(x,z)
						x=x+32*dx
						z=z+32*dz
						Spring.SetUnitMoveGoal(u,x,y,z)
						MovedLast[u]=true
					elseif MovedLast[u] then
						Spring.GiveOrderToUnit(u,CMD.STOP,{},{})
						MovedLast[u]=false
					end
				end
				if jx and jy and jz then
					Spring.GiveOrderToUnit(u,GG.CustomCommands.GetCmdID("CMD_JUMP"),{jx,jy,jz},{})
				end
			end
		end
		RecMsg={}

		if frame>LastHeroesCheck+139 then
			LastHeroesCheck=frame
			-- Removing heroes control from players that went inactive or spec
			for u,p in pairs(utp) do
				_,active,spectator,team=Spring.GetPlayerInfo(p)
				if not active then
					Spring.SendMessage("Player["..p.."] lost connection: removing player["..p.."]'s control over "..UnitDefs[Spring.GetUnitDefID(u)].humanName)
					ptf[p]=nil
					ptu[p]=nil
					utp[u]=nil
					MovedLast[u]=nil
				elseif spectator then
					Spring.SendMessage("<PLAYER"..p.."> is now spectating: removing <PLAYER"..p..">'s control over "..UnitDefs[Spring.GetUnitDefID(u)].humanName)
					ptf[p]=nil
					ptu[p]=nil
					utp[u]=nil
					MovedLast[u]=nil
				elseif team~=Spring.GetUnitTeam(u) then
					Spring.SendMessage("<PLAYER"..p.."> switched to team["..team.."]: removing <PLAYER"..p..">'s control over team["..Spring.GetUnitTeam(u).."]".." "..UnitDefs[Spring.GetUnitDefID(u)].humanName)
					ptf[p]=nil
					ptu[p]=nil
					utp[u]=nil
					MovedLast[u]=nil
				end
			end
			-- Give heroless players control over playerless heroes
			for _,team in ipairs(Spring.GetTeamList()) do
				for _,u in ipairs(Spring.GetTeamUnitsByDefs(team,hero)) do
					if not utp[u] and not Spring.GetUnitIsDead(u) then
						GiveHero(u,team)
					end
				end
			end
		end

	end


	function gadget:UnitCreated(u,ud,team)
		if isHero[ud] then
			LastHeroesCheck=-8888
		end
	end


	function gadget:UnitDestroyed(u,ud,team,atk,atkd,atkteam)
		if utp[u] then
			ScoreBoard[utp[u]].deaths=(ScoreBoard[utp[u]].deaths or 0)+1
			ScoreBoard[utp[u]].kills=0
			Spring.SendMessage("<PLAYER"..utp[u].."> lost "..UnitDefs[ud].humanName.."!")
			ptu[utp[u]]=nil
			utp[u]=nil
			MovedLast[u]=nil
		end
		if utp[atk] and not Spring.AreTeamsAllied(atkteam,team) then
			ScoreBoard[utp[atk]].kills=(ScoreBoard[utp[atk]].kills or 0)+1
			ScoreBoard[utp[atk]].totalkills=(ScoreBoard[utp[atk]].totalkills or 0)+1
		end
	end


	function gadget:AllowUnitTransfer(u,ud,oldteam,newteam,capture)
		if isHero[ud] then
			return false
		else
			return true
		end
	end


	function gadget:AllowUnitCreation(ud,b,team,x,y,z)
		if isHero[ud] and Spring.GetModOptions()["homf"] and Spring.GetModOptions()["homf"]~="0" then
			if #GetBetterPlayerList(team)-#Spring.GetTeamUnitsByDefs(team,hero)>0 then
				return true
			else
				return false
			end
		end
		return true
	end


	if not AllowOtherUnitsControl then
		function gadget:AllowCommand(unitID, unitDefID, unitTeam, cmdID, cmdParams, cmdOptions, cmdTag, synced)
			if (not synced) and #GetBetterPlayerList(team)>0 then
				return false
			end
			return true
		end
	end


	function gadget:RecvLuaMsg(msg,player)
		if string.sub(msg,1,12)=="Shoot'n'Run:" then
			local u=tonumber(string.match(msg,"U:(%d+)"))
			RecMsg[1+#RecMsg]=msg
		end
	end

else
--UNSYNCED

	local function GetTheOne()
		if SYNCED.Shoot_n_Run and SYNCED.Shoot_n_Run.ptu then
			local u=SYNCED.Shoot_n_Run.ptu[Spring.GetLocalPlayerID()]
			if u and Spring.ValidUnitID(u) then
				return u
			end
		end
		return nil
	end

	function gadget:Update()-- Is called more than once per frame
		local u=GetTheOne()
		if Spring.GetGameRulesParam("sb_gameMode") == "dev" then
			return
		end
		if u then
			Spring.SelectUnitArray({},false)
			local x,_,z=Spring.GetUnitPosition(u)
			local _,v,paused=Spring.GetGameSpeed()
			v = paused and 0 or v
			local vx,_,vz=Spring.GetUnitVelocity(u)
			local r=UnitDefs[Spring.GetUnitDefID(u)].maxWeaponRange
			Spring.SetCameraState({name=ta,mode=1,px=x+9*v*vx,py=0,pz=z+9*v*vz,flipped=-1,dy=-0.9,zscale=0.5,height=2*r,dx=0,dz=-0.45},1)
		else
			if Spring.GetModOptions()["homf"] and Spring.GetModOptions()["homf"]~="0" and not Spring.GetSpectatingState() then
				Spring.SelectUnitArray({},false)
				local x,y,z=Spring.GetTeamStartPosition(Spring.GetLocalTeamID())
				if x<0 or z<0 then
					x,z=Game.mapSizeX/2,Game.mapSizeZ/2
					y=Spring.GetGroundHeight(x,z)
				end
				Spring.SetCameraState({name=ta,mode=1,px=x,py=0,pz=z,flipped=-1,dy=-0.9,zscale=0.5,height=1200,dx=0,dz=-0.45},5)
			end
		end
	end

	local function Unsynced_GameFrame(caller,frame)
		local u=GetTheOne()
		if u then

			local MoveX=0
			local MoveZ=0
			local TargetX=nil
			local TargetY=nil
			local TargetZ=nil
			local TargetU=nil
			local FireA=false
			local FireB=false
			local FireC=false
			local SelfD=false
			local JumpX,JumpY,JumpZ=false,false,false

			-- Get mouse input
			local mx,my,LeftButton,MiddleButton,RightButton = Spring.GetMouseState()
			local _,pos = Spring.TraceScreenRay(mx,my,true,false)
			local kind,unit = Spring.TraceScreenRay(mx,my,false,false)
			if type(pos)=="table" and pos[1] and pos[3] then
				TargetX=math.floor(0.5+pos[1])
				TargetY=math.floor(0.5+pos[2])
				TargetZ=math.floor(0.5+pos[3])
			end
			if kind=="unit" then
				TargetU=unit
			else
				TargetU=nil
			end

			-- Check that hero keys were set
			if #(Spring.GetActionHotKeys("hero_west") or {})+ #(Spring.GetActionHotKeys("hero_east") or {})
				+#(Spring.GetActionHotKeys("hero_north") or {})+#(Spring.GetActionHotKeys("hero_south") or {})==0 then
				Spring.Echo("No hero key set! Forcing arrows and wasd!")
				Spring.SendCommands({"bind up hero_north"})
				Spring.SendCommands({"bind left hero_west"})
				Spring.SendCommands({"bind down hero_south"})
				Spring.SendCommands({"bind right hero_east"})
				Spring.SendCommands({"bind w hero_north"})
				Spring.SendCommands({"bind a hero_west"})
				Spring.SendCommands({"bind s hero_south"})
				Spring.SendCommands({"bind d hero_east"})
				Spring.SendCommands({"bind e hero_jump"})
				Spring.SendCommands({"bind space hero_jump"})
			end

			-- Get keyboard input
			for _,key in ipairs(Spring.GetActionHotKeys("hero_west")) do
				if Spring.GetKeyState(Spring.GetKeyCode(key)) then
					MoveX=MoveX-1
				end
			end
			for _,key in ipairs(Spring.GetActionHotKeys("hero_east")) do
				if Spring.GetKeyState(Spring.GetKeyCode(key)) then
					MoveX=MoveX+1
				end
			end
			for _,key in ipairs(Spring.GetActionHotKeys("hero_north")) do
				if Spring.GetKeyState(Spring.GetKeyCode(key)) then
					MoveZ=MoveZ-1
				end
			end
			for _,key in ipairs(Spring.GetActionHotKeys("hero_south")) do
				if Spring.GetKeyState(Spring.GetKeyCode(key)) then
					MoveZ=MoveZ+1
				end
			end
			for _,key in ipairs(Spring.GetActionHotKeys("hero_jump")) do
				if Spring.GetKeyState(Spring.GetKeyCode(key)) then
					if TargetX and TargetY and TargetZ then
						JumpX,JumpY,JumpZ=pos[1],pos[2],pos[3]
					end
				end
			end
			if Spring.GetKeyState(Spring.GetKeyCode('d')) then
				local alt,ctrl,meta,shift=Spring.GetModKeyState()
				if ctrl then
					SelfD=true
				end
			end

			-- Write message to synced
			local msg="Shoot'n'Run:"
			if u then
				msg=msg.." U:"..u
				if MoveX and MoveZ then
					msg=msg.." MX:"..MoveX.." MZ:"..MoveZ
				end
				if TargetU then
					msg=msg.." TU:"..TargetU
				elseif TargetX and TargetY and TargetZ then
					msg=msg.." TX:"..TargetX.." TY:"..TargetY.." TZ:"..TargetZ
				end
				if JumpX and JumpY and JumpZ then
					msg=msg.." JX:"..JumpX.." JY:"..JumpY.." JZ:"..JumpZ
				end
				if LeftButton then
					msg=msg.." LMB"
				end
				if RightButton then
					msg=msg.." RMB"
				end
				if SelfD then
					msg=msg.." SD"
				end
			end
			Spring.SendLuaRulesMsg(msg)
		end
	end

	function gadget:Initialize()
		gadgetHandler:AddSyncAction("Shoot_n_Run_Unsynced_GameFrame",Unsynced_GameFrame)
	end

	function gadget:MousePress(x,y,button)
		if Spring.GetGameRulesParam("sb_gameMode") == "dev" then
			return
		end
		if GetTheOne() then
			return true
		end
		return false
	end

	function gadget:MouseRelease(x,y,button)
		local u=GetTheOne()
		if u and button==2 then-- So, middle mouse button is 2 apparently
			local mx,my = Spring.GetMouseState()
			local _,pos = Spring.TraceScreenRay(mx,my,true,false)
			Spring.SendLuaRulesMsg("Shoot'n'Run: U:"..u.." JX:"..pos[1].." JY:"..pos[2].." JZ:"..pos[3])
			return true
		end
		return false
	end

end
