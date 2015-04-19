
--[[
   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 2 of the License, or
   (at your option) any later version.
   
   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.
   
   You should have received a copy of the GNU General Public License
   along with this program; if not, write to the Free Software
   Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
   MA 02110-1301, USA.
   
]]--


--This Section contains standalone functions to be executed as independent systems monitoring and handling lua-stuff
--mini OS Threads

--> Unit Statemachine
	function stateMachine(unitid, sleepTime,State, stateT)
	local time=0
	local StateMachine=stateT
	local stateStorage={}
		while true do
		
		if not stateStorage[State]then stateStorage[State]={} end
		
		State, stateStorage =StateMachine[State](unitid,time,stateStorage)
		Sleep(sleepTime)
		time=time+sleepTime
		end
	end




--> Gadget:missionScript expects frame, the missionTable, which contains per missionstep the following functions
-- e.g. [1]= {situationFunction(frame,TABLE,nr), continuecondtion(frame,TABLE,nr,boolsuccess), continuecondtion(frame,TABLE,nr,boolsuccess)}
-- in Addition every Functions Table contains a MissionMap which consists basically of a statediagramm starting at one
-- MissionMap={[1]=> {2,5},[2] => {1,5},[3]=>{5},[4]=>{5},[5]=>{1,5}}

	function missionHandler(frame,TABLE,nr)
	--wethere the mission is continuing to the next nr
	boolContinue=false
	--wether the mission has a Outcome at all
	boolSituationOutcome =TABLE[nr].situationFunction(frame,TABLE,nr)
	
	--we return nil if the situation has no defined outcome
	if not boolSituationOutcome then return end
	
		if not TABLE[nr].continuecondtion then
			boolContinue=true
		elseif type(TABLE[nr].continuecondtion)=='number'then	
			if frame > TABLE[nr].continuecondtion then boolsuccess=true end
		elseif type(TABLE[nr].continuecondtion)=='function'then	
			boolContinue=TABLE[nr].continuecondtion(frame,TABLE,nr,boolsuccess)
		end 
		
	if boolContinue==true then
		return TABLE[nr].continuecondtion(frame,TABLE,nr,boolsuccess)
	else
	return nr
	end
	
	end
--> jobfunc header jobFunction(unitID,x,y,z, Previousoutcome)  --> checkFuncHeader  checkFunction(unitID,x,y,z,outcome)
function getJobDone(unitID, dataTable, jobFunction, checkFunction,rest)
local dataT=dataTable
local spGetUnitPosition=Spring.GetUnitPosition
x,y,z=spGetUnitPosition(unitID)
outcome=false

	while checkFunction(unitID,dataT,x,y,z,outcome) ==false do
	x,y,z=spGetUnitPosition(unitID)
	outcome=jobFunction(unitID, dataT, x,y,z, outcome)
	Sleep(rest)
	end

end

function cegDevil(cegname, x,y,z,rate, lifetimefunc, endofLifeFunc,boolStrobo, range, damage, behaviour)

knallfrosch=function(x,y,z,counter,v)
				if counter % 120 < 60 then -- aufwärts
					if v then
					return x*v.x,y*v.y,z*v.z, v
					else
					return x,y,z, {x=math.random(1,1.4)*randSign(),y=math.random(1,2),z=math.random(1,1.4)*randSign()}
					end
				elseif Spring.GetGroundHeight(x,z) -y < 10 then --rest
				return x,y,z
				else --fall down
					if v and v.y < 0 then
						return x*v.x,y*v.y,z*v.z, v
					else
					return x,y,z, {x=math.random(1,1.1)*randSign(),y=math.random(1,2),z=math.random(1,1.4)*randSign()}
					end
			
				end
			end
functionbehaviour=behaviour or knallfrosch
time=0			
local SpawnCeg=Spring.SpawnCeg
v= {x=0,y=0,z=0}

	while lifetimefunc(time)==true do
	x,y,z,v=functionbehaviour(x,y,z,time,v)
		
		if boolStrobo==true then
		dx,dy,dz=randVec()
		SpawnCeg(cegname,x,y,z,dx,dy,dz,range,damage)
		else
		SpawnCeg(cegname,x,y,z,0,1,0,range,damage)
		end
	
	time=time+rate
	Sleep(rate)
	end

endofLifeFunc(x,y,z)
end


--PieceDebug loop
	function PieceLight(unitID, piecename,cegname)
		while true do
		x,y,z=Spring.GetUnitPiecePosDir(unitID,piecename)
		Spring.SpawnCEG(cegname,x,y+10,z,0,1,0,50,0)
		Sleep(250)
		end
	end
	
--> AmphibMovementThread
function AmphibMoveThread(unitid
						 ,PivotPoints
						 ,pieces
						 ,updateCycle
						 ,moveRatio
						 ,nlswimAnimation
						 ,nlstopSwimAnimation
						 ,nloutOfWaterAnimation
						 ,nlbackIntoWaterAnimation
						 ,nlwalkAnimation
						 ,nlstopWalkAnimation)
vx,vy,vz=Spring.GetUnit							
local swimAnimation          =nlswimAnimation   
local stopSwimAnimation      =nlstopSwimAnimation
local outOfWaterAnimation    =nloutOfWaterAnimation
local backIntoWaterAnimation =nlbackIntoWaterAnimation
local walkAnimation          =nlwalkAnimation
local stopWalkAnimation		 =nlstopWalkAnimation								
local spGetUnitPosition =	Spring.GetUnitPosition

boolInWater= function ()
				x,y,z=spGetUnitPosition(unitID)
				h=Spring.GetGroundHeight(x,z)
				if h > 0 then return false else return true end
				end

boolMoving= function (ox,oy,oz)
			x,y,z=spGetUnitPosition(unitID)
			return math.abs(ox-x)+math.abs(oz-z)+math.abs(oy-y) > 0
			end


	while true do
		while boolInWater()==true do
			ox,oy,oz=spGetUnitPosition(unitID)
			Sleep(math.floor(updateCycle/2))
			if  boolMoving(ox,oy,oz)==true then
				swimAnimation(PivotPoints,pieces)
			else 
			Sleep(math.floor(updateCycle/2))
			stopSwimAnimation(PivotPoints,pieces)
			end
		Sleep(math.ceil(updateCycle/2))
		
		end
		
	outOfWaterAnimation(PivotPoints,pieces)
		while boolInWater()==false do
			ox,oy,oz=spGetUnitPosition(unitID)
			Sleep(math.floor(updateCycle/2))
			if  boolMoving(ox,oy,oz)==true then
			walkAnimation(PivotPoints,pieces)
			else 
			Sleep(math.floor(updateCycle/2))
			stopWalkAnimation(PivotPoints,pieces)
			end
		Sleep(math.ceil(updateCycle/2))
		end
	backIntoWaterAnimation(PivotPoints,pieces)
	Sleep(50)
	end

end	
	

--> genericOS 
function genericOS(unitID, dataTable,jobFunctionTable, checkFunctionTable,rest)
local checkFunctionT	=checkFunctionTable
local jobFunctionT		=jobFunctionTable
local dataT				=dataTable
local spGetUnitPosition=Spring.GetUnitPosition

x,y,z=spGetUnitPosition(unitID)
outcomeTable=iniT(#jobFunctionT,false)
boolAtLeastOneNotDone=true
	while boolAtLeastOneNotDone ==true do
	x,y,z=spGetUnitPosition(unitID)
		for i=1,#jobFunctionT do
		outcomeTable[i]=jobFunctionT[i](unitID,x,y,z, outcomeTable[i],dataT)
		Sleep(rest)
		end
	boolAtLeastOneNotDone=true
		for i=1,#checkFunctionT do
		boolAtLeastOneNotDone= checkFunction(unitID,x,y,z,outcomeTable[i]) and boolAtLeastOneNotDone
		Sleep(rest)
		end
	
	end
end

-->Turn Piece into various diretions within range
function randomRotate(Piecename,axis, speed, rangeStart,rangeEnd)
	while true do
	Turn(Piecename,axis,math.rad(math.random(rangeStart,rangeEnd)),speed)
	WaitForTurn(Piecename,axis)
	Sleep(1000)
	end

end

-->plays the sounds handed over in a table 
function playSoundByUnitTypOS(unitID,loudness,SoundNameTimeT)
local SoundNameTimeTable=SoundNameTimeT
	 unitdef=Spring.GetUnitDefID(unitID)
	
	while true do
	dice=math.random(1,#SoundNameTimeTable)

	PlaySoundByUnitType(unitdef, SoundNameTimeTable[dice].name,loudness, SoundNameTimeTable[dice].time, 1)
	Sleep(1000)
	end
end

--===================================================================================================================
--Game specific functions
--> creates a table from names to check unittypes against
function getTypeTable(UnitDefNames,StringTable)
local Stringtable=StringTable
retVal={}
	for i=1,#Stringtable do
	assert(UnitDefNames[Stringtable[i]], "Error: Unitdef of Unittype "..Stringtable[i].. " does not exists")
	retVal[UnitDefNames[Stringtable[i]].id]=true
	end
return retVal
end

--> JW specific function returning the factorys of the game
function getFactoryTypeTable(UnitDefNames,IWant)
FactoryTypes={}	


if IWant=="c" then
FactoryTypes[UnitDefNames["fclvlone"].id]=true
FactoryTypes[UnitDefNames["fclvl2"].id]=true
FactoryTypes[UnitDefNames["condepot"].id]=true
return FactoryTypes
end

if IWant=="j" then
FactoryTypes[UnitDefNames["jtrafactory"].id]=true
FactoryTypes[UnitDefNames["eggstackfac"].id]=true
FactoryTypes[UnitDefNames["jmovingfac1"].id]=true
return FactoryTypes
end

--I want it all
FactoryTypes[UnitDefNames["jtrafactory"].id]=true
FactoryTypes[UnitDefNames["eggstackfac"].id]=true
FactoryTypes[UnitDefNames["jmovingfac1"].id]=true
FactoryTypes[UnitDefNames["fclvlone"].id]=true
FactoryTypes[UnitDefNames["fclvl2"].id]=true
FactoryTypes[UnitDefNames["condepot"].id]=true
return FactoryTypes

end

function getPyroProofTable(UnitDefNames)
FireProofTypes={}
FireProofTypes[UnitDefNames["css"].id]=true
FireProofTypes[UnitDefNames["jfireflower"].id]=true
FireProofTypes[UnitDefNames["citadell"].id]=true
FireProofTypes[UnitDefNames["beanstalk"].id]=true
return FireProofTypes
end

function getTreeTypeTable(UnitDefNames)
FactoryTypes={}
FactoryTypes[UnitDefNames["jtree2"].id]=true
FactoryTypes[UnitDefNames["jtree2activate"].id]=true
FactoryTypes[UnitDefNames["jtree2dummy"].id]=true
FactoryTypes[UnitDefNames["jtree3"].id]=true
FactoryTypes[UnitDefNames["jtree3dummy"].id]=true
FactoryTypes[UnitDefNames["jtree41"].id]=true
FactoryTypes[UnitDefNames["jtree42"].id]=true
FactoryTypes[UnitDefNames["jtree43"].id]=true
FactoryTypes[UnitDefNames["jtree44"].id]=true
FactoryTypes[UnitDefNames["jtree45"].id]=true
FactoryTypes[UnitDefNames["jtree46"].id]=true
FactoryTypes[UnitDefNames["jtree47"].id]=true
FactoryTypes[UnitDefNames["jtree"].id]=true
return FactoryTypes
end
--===================================================================================================================
--functions
-->returns the Negated Value
function N(value)
value=-1*value
return value
end

-->returns the 2 norm of a vector
function distance(x,y, z)
return math.sqrt(x*x+y*y+z*z)
end

-->returns the Distance between two units
function GetUnitDistance(idA, idB)
x,y,z =Spring.GetUnitPosition(idA)
xb,yb,zb=Spring.GetUnitPosition(idB)
return GetTwoPointDistance(x,y,z,xb,yb,zb)
end

-->returns Distance between two points
function GetTwoPointDistance(x,y,z,ox,oy,oz)
x=x-ox
y=y-oy
z=z-oz
return  math.sqrt(x*x+y*y+z*z)
end

--> gives a close hunch at the distance and avoids expensive sqrt math
function approxDist(x,y,z)
x=(x*x+y*y+z*z)-1
xs=x*x
return ((((1+ (x/2))+ ((xs)/-8))+((xs*x)/16))+((xs*xs*5)/-128))
end

-->adds a num Table to a num Table
function TAddT(OrgT,T)
	for i=1,#T do
	OrgT[#OrgT+i]=T[i]
	end
return OrgT
end

--> Converts to points, to a degree
function convPointsToDeg(ox,oz,bx,bz)
	if not bx then --orgin cleaned point
	return math.atan2(ox,oz)
		else
		bx=bx-ox
		bz=bz-oz
		return math.atan2(bx,bz)
		end
end

-->Turn a piece towards a random direction
function turnPieceRandDir(piecename,speed, LIMUPX,LIMLOWX,LIMUPY,LIMLOWY,LIMUPZ,LIMLOWZ)
	if not limUpX then
	Turn(piecename,x_axis,math.rad(math.random(-360,360)),speed)
	Turn(piecename,y_axis,math.rad(math.random(-360,360)),speed)
	Turn(piecename,z_axis,math.rad(math.random(-360,360)),speed)
	else
	Turn(piecename,x_axis,math.rad(math.random(LIMLOWX,LIMUPX)),speed)
	Turn(piecename,y_axis,math.rad(math.random(LIMLOWY,LIMUPY)),speed)
	Turn(piecename,z_axis,math.rad(math.random(LIMLOWZ,LIMUPZ)),speed)
	end
end


-->Reset a Piece at speed
function resetPiece(piecename,speed,boolWaitForIT)
Turn(piecename,x_axis,0,speed)
Turn(piecename,y_axis,0,speed)
Turn(piecename,z_axis,0,speed)

Move(piecename,x_axis,0,speed)
Move(piecename,y_axis,0,speed)
Move(piecename,z_axis,0,speed,true)
if boolWaitForIT then 
WaitForTurn(piecename,1)
WaitForTurn(piecename,2)
WaitForTurn(piecename,3)
end



end

function equiTurn(p1,p2,axis,deg,speed)
Turn(p1,axis,math.rad(deg),speed)
Turn(p2,axis,math.rad(-1*deg),speed)
end

function tP(piecename,x_val,y_val,z_val,speed,boolWaitForIT)

Turn(piecename,x_axis,math.rad(x_val),speed)
Turn(piecename,y_axis,math.rad(y_val),speed)
Turn(piecename,z_axis,math.rad(z_val),speed)
if boolWaitForIT then 
WaitForTurn(piecename,x_axis)
WaitForTurn(piecename,y_axis)
WaitForTurn(piecename,z_axis)
end
end

function tPrad(piecename,x_val,y_val,z_val,speed)
Turn(piecename,x_axis,x_val,speed)
Turn(piecename,y_axis,y_val,speed)
Turn(piecename,z_axis,z_val,speed)
end

function mP(piecename,x_val,y_val,z_val,speed)
Move(piecename,x_axis,x_val,speed)
Move(piecename,y_axis,y_val,speed)
Move(piecename,z_axis,z_val,speed)
end


function TurnTowardsWind(piecename,speed,offset)
offSet=offset or 0
dx,dy,dz=Spring.GetWind()
headRad=math.atan2(dx,dz)
Turn(piecename,y_axis,headRad+offSet,speed)
end

function spinT(Table, axis,speed,deg, degup)
if not degup then
	for i=1,#Table do
	Spin(Table[i],axis,math.rad(deg),speed)
	end
else
	for i=1,#Table do
	Spin(Table[i],axis,math.rad(math.random(deg,degup)),speed)
	end
end

end

function stopSpinT(Table,axis, speed)
	for i=1,#Table do
	StopSpin(Table[i],axis,speed)
	end

end

-->Creates basically a table of piecenamed enumerated strings
function makeTableOfPieceNames(name, nr,startnr)
T={}
start=startnr or 1

	for i=start,nr do
	namecopy=name	
	namecopy=namecopy..i
	T[i]=namecopy
	end
return T
end

function resetPieceDir(piecename,speed)
Turn(piecename,x_axis,0,speed)
Turn(piecename,y_axis,0,speed)
Turn(piecename,z_axis,0,speed)


end

-->Reset a Table of Pieces at speed
function reseT(tableName,speed)
lspeed=speed or 0

	for i=1,#tableName do
	resetPiece(tableName[i],lspeed)
	end
end

function recReseT(Table,speed)
if type(Table)=="table" then 
for k,v in pairs(Table) do
recReseT(v,speed)
end
elseif type(Table)=="number" then
resetPiece(Table,speed)
end
end

function getUnitSide(unitID)
teamid=Spring.GetUnitTeam(unitID)
teamID,    leader,      isDead,    isAiTeam, side,  allyTeam,  customTeamKeys,  incomeMultiplier=Spring.GetTeamInfo(teamid)
return side
end

-->Moves a UnitPiece to Position at speed
function MovePieceToPos(piecename, X,Y,Z,speed)

Move(piecename,x_axis,X,speed,true)
Move(piecename,y_axis,Y,speed,true)
Move(piecename,z_axis,Z,speed,true)	

end

-->Moves a UnitPiece to a UnitPiece at speed
function MovePieceToPiece(piecename, piecenameB,speed)
if not piecenameB or not piecename then return end
bx,by,bz=Spring.GetUnitPiecePosDir(unitID,piecenameB)
x,y,z=Spring.GetUnitPiecePosDir(unitID,piecename)

Move(piecename,x_axis,bx-x,speed)
Move(piecename,y_axis,by-y,speed)
Move(piecename,z_axis,bz-z,speed,true)	

end


-->Turns a Piece towards a direction 
function TurnPieceTowards(piecename,x,y,z,speed)

	Turn(piecename,x_axis,math.rad(x),speed)
	Turn(piecename,y_axis,math.rad(y),speed)
	Turn(piecename,z_axis,math.rad(z),speed,true)	
	
end

-->Turn a Piece towards another Piece 
function TurnPieceTowardsPiece(piecename,pieceB,speed)
		x,y,z=Spring.GetUnitPiecePosition(unitID,piecename)
		px,py,pz=Spring.GetUnitPiecePosition(unitID,pieceB)
		px,py,pz=x-px,y-py,z-pz
		dx,dy,dz=math.rad(math.atan2(dy,dz)),math.rad(math.atan2(px,pz)),math.rad(math.atan2(dy,dx))
			if py then
			TurnPieceTowards(piecename,dx,dy,dz,speed)
			end

end

--> Turns a Piece into the Direction of the coords given (can take allready existing piececoords for a speedup
function TurnPieTowardsPoint (piecename, x,y,z,Speed, tpx,tpy,tpz)
pvec={x=0,y=0,z=0}
px,py,pz,pvec.x,pvec.y,pvec.z =Spring.GetUnitPiecePosDir(unitID,piecename) 
pvec=normalizeVector(pvec)

vec={}
vec.x,vec.y,vec.z=x-px,y-py,z-pz
v=normalizeVector(v)
v=VsubV(v,pvec)
v=normalizeVector(v)
tPrad(math.atan2(vec.y,vec.z),math.atan2(vec.x,vec.z),math.atan2(vec.x,vec.y),Speed)
end


--> Moves a Piece to a WorldPosition relative to the Units Position
function MovePieceToRelativeWorldPos(id,piecename, relX,relY,relZ,speed)
x,y,z=Spring.GetUnitPosition(id)
x,y,z=relX-x,y-relY,relZ-z
Move(piecename,x_axis,x,speed)
Move(piecename,y_axis,y,speed)
Move(piecename,z_axis,z,speed,true)	

end

-->Returns randomized Boolean
function maRa()
return math.random(0,1)==1 
end

-->Returns not nil if random
function maRo()
if math.random(0,1)==1 then return true else return end
end

-->Move with a speed Curve
function moveSpeedCurve(piecename, axis, NumberOfArgs, now, timeTotal , distToGo, Offset,...)
--!TODO calcSpeedUpId from functionkeys,check calculations for repetitons and store that key in to often as result in GG
--should handle all sort of equations of the type 0.3*x^2+0.1*x^1+offset
-- in our case that would be [2]=0.3 ,[1]=0.1 and so forth

ArgFactorTable={}
NrCopy=NumberOfArgs
      for _, factor in pairs(arg) do
                ArgFactorTable[NrCopy]=factor	  
				NrCopy=NrCopy-1
	  end

--init tangent table
tangentTable={n=#ArgFactorTable-1}


--first derivative 
--http://en.wikipedia.org/wiki/Derivative
for i=table.getn(tangentTable), 1, -1 do
tangentTable[i]=(i+1)*ArgFactorTable[i+1]
end
	  
	Totalspeed=Offset
	for i=1, NumberOfArgs-1 do
	Totalspeed=Totalspeed+tangentTable[i]*(now^i)
	end
	
Move(piecename, axis, distToGo, Totalspeed)
end
--> Drops a piece to the ground
function DropPieceToGround(unitID,piecename,speed, boolWait,boolHide, ExplodeFunction,SFXCOMBO)
x,y,z=Spring.GetUnitPiecePosition(unitID,piecename)
MoveUnitPieceToGroundPos(unitID,piecename,x,z,speed, 5)

if boolWait then WaitForMove(piecename,y_axis) end

if boolHide then Hide(piecename) end	

if ExplodeFunction then ExplodeFunction(piecename,SFXCOMBO) end
end

--> Destroys A Table of Units
function DestroyTable(T, boolSelfd, boolReclaimed, condFunction,unitID)
if T then 
	for i=1,#T, 1 do
		if condFunction(T[i])==true then
		Spring.DestroyUnit(T[i],boolSelfd,boolReclaimed, unitID)
		end
	end
end
end


-->Moves a Piece to a Position on the Ground in UnitSpace
function MoveUnitPieceToGroundPos(unitID,piecename, X,Z,speed,offset)
if not piecename then return error("No piecename given") end
loffset=offset or 0
x,globalHeightUnit,z=Spring.GetUnitPosition(unitID)
Move(piecename,x_axis,X,0)
Move(piecename,z_axis,Z,0,true)	
x,y,z,_,_,_=Spring.GetUnitPiecePosDir(unitID,piecename)
	myHeight=Spring.GetGroundHeight(x,z)
	heightdifference=math.abs(globalHeightUnit-myHeight)
		if myHeight < globalHeightUnit then heightdifference=-heightdifference end
Move(piecename,y_axis,heightdifference+loffset,speed,true)
end

-->Moves a Piece to WaterLevel on the Ground in UnitSpace
function KeepPieceAfloat(unitID,piecename,speed,offset)
if not piecename then return error("No piecename given") end
loffset=offset or 0
x,globalHeightUnit,z=Spring.GetUnitPosition(unitID)

x,y,z,_,_,_=Spring.GetUnitPiecePosDir(unitID,piecename)
	myHeight=0
	heightdifference=math.abs(globalHeightUnit-myHeight)
		if myHeight < globalHeightUnit then heightdifference=-heightdifference end
Move(piecename,y_axis,heightdifference+loffset,speed,true)
end

-->Paint a Piece Pattern 
function PaintPatternPieces(ListOfPieces, ListOfCoords,sx,sy,sz)
prevx,prevy,prevz=sx,sy,sz


	MovePieceToPos(ListOfPieces[1],ListOfCoords[1].x,ListOfCoords[1].y,ListOfCoords[i].z)
	TurnPieceTowards(ListOfPieces[1],sx,sy,sz,0)
	prevx,prevy,prevz=ListOfCoords[1].x,ListOfCoords[1].y,ListOfCoords[i].z
	

	for i=2,#ListOfCoords-1 do
	MovePieceToPos(ListOfPieces[i],ListOfCoords[i].x,ListOfCoords[i].y,ListOfCoords[i].z)
	TurnPieceTowardsPiece(ListOfPieces[i],ListOfPieces[i-1],0)
	end

end

-->Moves a Piece to a Position on the Ground in Worldspace
function MoveUnitPieceToRelativeWorldPos(unitID,piecename, relX,relZ,speed,offset)
x,globalHeightUnit,z=Spring.GetUnitPosition(unitID)
x,z=relX-x,relZ-z
Move(piecename,x_axis,x,0,true)
Move(piecename,z_axis,z,0,true)	
x,y,z,_,_,_=Spring.GetUnitPiecePosDir(unitID,piecename)
	myHeight=Spring.GetGroundHeight(x,z)
	heightdifference=math.abs(globalHeightUnit-myHeight)
	if myHeight < globalHeightUnit then heightdifference=-heightdifference end
Move(piecename,y_axis,heightdifference+offset,speed,true)
end


-->Copys a table
function tableCopy(orig)
	local orig_type=type(orig)
	local copy
		if orig_type=='table' then
		copy={}
		for orig_key,orig_value in next, orig, nil do
		copy[tableCopy(orig_key)]=tableCopy(orig_value)
		end
		setmetatable(copy,tableCopy(getmetatable(orig)))
			else
			copy=orig
			end
	return copy
	end
	
	

-->Prints Out a Table	
function printOUT(tmap,squareSideDimension)
	local map={}
	map=tmap
	step=8
	if squareSideDimension~= nil and squareSideDimension < 129 then step=1 end
		for x=2,#map,step do
		StringToConcat=""
			for z=2,#map,step do
		
				
				if map[x][z] == nil  then 
						StringToConcat=StringToConcat.." X "
				elseif  map[x][z] == 0 then
						StringToConcat=StringToConcat.." 0 "
				elseif map[x][z] == false or map[x][z] == true then
						StringToConcat=StringToConcat.." € "
				else
				val=math.ceil(map[x][z])		
						if val < 10 and val >=0 then
					
						StringToConcat=StringToConcat.." "..val.." "
							elseif val >9 and val < 100 then
							StringToConcat=StringToConcat.." "..val
								elseif val > 99 then
								StringToConcat=StringToConcat..val
								else
								StringToConcat=StringToConcat..val
								end
						end		
					
				
			end	
			Spring.Echo("::"..StringToConcat.."::")
		end

	end	
	
	
function ANHINEG(value)
if value <0 then
value=M(value)
end
return value
end

function PP(value)
value=value+1
return value
end
--Bit Operators -Great Toys for the BigBoys
function SR(value,shift)
reSulTan=math.bit_bits(value,shift)
return reSulTan
end 

function SL(value,shift)
reSulTan=math.bit_bits(value,M(shift))
return reSulTan
end

function  AND(value1,value2)
reSulTan=math.bit_and(value1,value2)
return reSulTan
end

function OR(value1,value2)
reSulTan=math.bit_or(value1,value2)
return reSulTan
end

function XOR(value1,value2)
reSulTan=math.bit_xor(value1,value2)
return reSulTan
end


function INV(value)
reSulTanane=math.bit_inv(value)
return value
end

-->Get the Ground Normal, uses a handed over function and returns a corresponding Table
function getGroundMapTable(Resolution, HandedInFunction)
ReT={}
	for x=1, Game.mapSizeX, Resolution do
	ReT[x]={}
		for y=1, Game.mapSizeY, Resolution do
		dx,dy,dz=Spring.GetGroundNormal(x,y)
		ReT[x][y]=Helperfunction(dx,dy,dz)
		end
	end
return ReT
end

-->Generalized map processing Function
-->Get the Ground Normal, uses all handed over functions for processing and returns a corresponding Table
function doForMapPos(Resolution,...)
for k,v in pairs(arg) do if type(v)~="function" then return Spring.Echo(" Argument is not a processing function") end end

ReT={}
	for x=1, Game.mapSizeX, Resolution do
	ReT[x]={}
		for y=1, Game.mapSizeY, Resolution do
			res={}
			for k,v in pairs(arg) do
			res=arg(x,y,res) 
			end
			ReT[x][y]=res 
		end
	end
return ReT
end

-->Rotates a point around another Point
function drehMatrix (x, y, zx, zy, degree)
x= x-zx
y=y-zy
x=(math.cos(degree)+ (-1*math.sin(degree)))*x
y=(math.cos(degree)+ (math.sin(degree)))*y

	IntCastX=x%1
	x=x-IntCastX
	IntCastY=y%1
	y=y-IntCastY

x=x+zx
y=y+zy
return x,y
end

-->Checks wether a point is within a triangle
function pointWithinTriangle(x1,y1,x2,y2,x3,y3,xt,yt)

--triAngleDeterminates
x1x2=0.5*(x1*y2-x2*y1)
x2x3=0.5*(x2*y3-x3*y2)
x3x1=0.5*(x3*y1-x1*y3)
--pointAngleDeterminates
x1xt=0.5*(x1*yt+xt*y1)
x2xt=0.5*(x2*yt+xt*y2)
x3xt=0.5*(x3*yt+xt*y3)

det1=x1x2+x1xt+x2xt--x1x2xt
det2=x2xt+x3xt+x2x3--x3x2xt
det1=x3xt+x3x1+x1xt--x1x3xt
detSum=det1+det2+det3
        if detSum<0 then
        --Point is outside of triAngle
        return true
        end

                if detSum >= 0 then
                --Point is inside of triAngle
                return false
                end

end

--> checks wether a value with teshold is within range of a second value
function withinRange(value1,value2, treShold)
if value1*treShold > value2 or value1*(1-treShold) <treShold then
return true
	else
	return false
	end
end

function valComPair(value1,treShold)
if value1 <0 then
value1=M(value1)
end

if value1 < value1*treShold and value1 > value1*(1-treShold) then
return true
	else
	return false
	end
end
--> Checks wether a Point is within a six sided polygon
function sixPointInsideDetAlgo (x1,y1,x2,y2,x3,y3,x4,y4,x5,y5,x6,y6,xPoint,yPoint)
boolInside=true
detSum=0
	for i=0, 6, 1 do
	tempdet=0.5*((x((i+1)%7))* (y((i+2)%7))+(x((i+2)%7))*(y((i+1)%7)))
	detSum=detSum+tempdet
	end

	if detSum>=0 then 
	boolInside=false
	end

return boolInside
end
-->Gets the Height of a Unit
function getUnitHeight(UnitId)
_,y,_=Spring.GetUnitPosition(unitID)
return y
end

-->multiplies a 3x1 Vector with a 3x3 Matrice
function vec3MulMatrice3x3(vec,Matrice)
return {x=Matrice[1][1]*vec.x+Matrice[1][2]*vec.y,Matrice[1][3]*vec.z,
		y=Matrice[2][1]*vec.x+Matrice[2][2]*vec.y,Matrice[2][3]*vec.z,
		z=Matrice[3][1]*vec.x+Matrice[3][2]*vec.y,Matrice[3][3]*vec.z,
		}
end
--RawMatrice
-- {
-- [1]={[1]=,[2]=,[3]=,},
-- [2]={[1]=,[2]=,[3]=,},
-- [3]={[1]=,[2]=,[3]=,}
-- }
function YRotationMatrice(Deg)
return {
		[1]={[1]=math.cos(Deg),		[2]=0,		[3]=math.sin(Deg)*-1,},
		[2]={[1]=0,					[2]=1,		[3]=0},
		[3]={[1]=math.sin(Deg),		[2]=0,		[3]=math.cos(Deg),	}
		}
end

function XRotationMatrice(Deg)
return {
							[1]={[1]=1,				[2]=0,				[3]=0				},
							[2]={[1]=0,				[2]=math.cos(Deg),	[3]=math.sin(Deg)*-1},
							[3]={[1]=0,				[2]=math.sin(Deg),	[3]=math.cos(Deg)	}
							}
end
function ZRotationMatrice(Deg)
return {
							[1]={[1]=math.cos(Deg),	[2]=math.sin(Deg)*-1,	[3]=0,},
							[2]={[1]=math.sin(Deg),	[2]=math.cos(Deg),		[3]=0,},
							[3]={[1]=0,				[2]=0,					[3]=1,}
							}
end

function rotateVecDegX(vec, DegX)
	tempDegRotY=math.asin(vec.x/(math.sqrt(vec.x^2+vec.z^2)))
	
	-- y-axis
	vec=vec3MulMatrice3x3(vec,YRotationMatrice(tempDegRotY*-1))
	
	tempDegRotZ=math.asin(vec.y/math.sqrt(vec.x^2 +vec.z^2))
	--z-axis
	vec=vec3MulMatrice3x3(vec,YRotationMatrice(tempDegRotZ*-1))
	--actual Rotation around the x-axis
	vec=vec3MulMatrice3x3(vec,XRotationMatrice(DegX))
	
	--undo z-axis
	vec=vec3MulMatrice3x3(vec,YRotationMatrice(tempDegRotZ))
	
	-- undo y-axis
	vec=vec3MulMatrice3x3(vec,YRotationMatrice(tempDegRotY))
	return vec	
end
	
	

-->gets the original Mapheight
function getHistoricHeight(UnitId)
tempX,tempY,tempZ=Spring.GetUnitPosition(UnitId)
tempY=Spring.GetGroundOrigHeight(tempX,tempZ)
return tempY
end
--> returns true if point is within range -returns false if it is on the outside
local previousResult
local previousCubic
local rangeOfOld = -1


--> faster way of finding out wether a point is within a circle
function isWithinCircle(circleRange,xCoord,zCoord)
newCubic=0
	if rangeOfOld == circleRange then
	newCubic=previousCubic
	else
	newCubic= 0.7071067811865475*circleRange
	previousCubic=newCubic
	end

negCircleRange=-1*circleRange

	--checking the most comon cases   |Coords Outside the Circlebox
			if xCoord > circleRange or xCoord < negCircleRange then
			return false
			end

					if zCoord >circleRange or zCoord <negCircleRange then
					return false
					end

negNewCubic=-1* newCubic


							--checking everything within the circle box
							if (xCoord < newCubic and xCoord > negNewCubic) and (zCoord < newCubic and zCoord > negNewCubic) then
							return true
							end


-- very few cases make it here.. to the classic, bad old range compare
if math.sqrt(xCoord^2 +zCoord^2) < circleRange then 
return true 
else 
return false
end

end


--conditonal default :executes the most often used option
conditionFullFilled={}
--this Table contains the functions of the condtion.. each either returning a true (executed correct), a false (conditions not fullfilled), or a nil (reached, but not executed correct)

---------------------------------------
--initialising the number of cases
intNrOfCases=2
intCases={}
for i=1,intNrOfCases,1 do
intCases[i]={}
intCases[i][1]=0
intCases[i][2]=false
intCases[i][3]=false
end
-----------------------------------------
-->Turns a piece in the speed necessary to arrive after x Milliseconds
function turn(piecename,axis,degree,timeInMs,boolWait)
timeInMs=timeInMs/1000
Speed=Degree/timeInMs
if degree < 180 or degree < -180 then
Turn(piecename,axis,math.rad(degree),Speed)
if boolWait==true then WaitForTurn(piecename,axis) end
	else
	deg=degree%180
	m=1
	if degree < 0 then m=-1 end
	Turn(piecename,axis,math.rad(179*m),Speed)
	if boolWait==true then WaitForTurn(piecename,axis) end
	Turn(piecename,axis,math.rad(degree),Speed)
	if boolWait==true then WaitForTurn(piecename,axis) end
	end
end
-->Packs Values into Pairs
function getPairs(values)
    xyPairs = {}
    for i=1,#values,2 do 
        v = {x=values[i], y=values[i+i] }
        table.insert(xyPair, v)
    end
    return xyPairs
end


-->encapsulates a function, stores arguments given, chooses upon returned nil, 
--	the most often chosen argument
function heuristicDefault(fooNction,fname, teamID, ...)

if not  GG[fname] then  GG[fname]={} end
if not GG[fname][teamID] then GG[fname][teamID] ={} end

local heuraTable= GG[fname][teamID] 
ArgumentCounter=1
	for k,v in pairs(arg) do
	if not heuraTable[ArgumentCounter]then heuraTable[ArgumentCounter]={}end
	if not heuraTable[ArgumentCounter][v] then heuraTable[ArgumentCounter][v]=1 else heuraTable[v]=heuraTable[ArgumentCounter][v]+1  end
	ArgumentCounter=ArgumentCounter+1
	end

results=fooNction(args)

	if not results  then
	--devalue current Arguments
		ArgumentCounter=1
		for k,v in pairs(arg) do
		heuraTable[ArgumentCounter][v]=heuraTable[ArgumentCounter][v]-1  
		ArgumentCounter=ArgumentCounter+1
		end

	--call the function with the most likely arguments
	newWorkingSet={}
		ArgumentCounter=1
		for k,v in pairs (arg) do
		highestVal,highestCount=0,0
			for i,j in pairs ( heuraTable[ArgumentCounter]) do
				if heuraTable[ArgumentCounter][v] > highestCount then
				highestCount= heuraTable[ArgumentCounter][v] 
				highestVal= v
				end 
			end
		table.insert(newWorkingSet,highestVal)
		ArgumentCounter=ArgumentCounter+1
		end
	results=fooNction(newWorkingSet)
	Spring.Echo("FallBack::Heuristic Default")
	assert(results, "Heuristic Default has inssuficient working samples.Returns Nil")
	GG[fname][teamID]=heuraTable
	return results
		else
		GG[fname][teamID]=heuraTable
		return results
		end
end 

-->generates a Pieces List Keyed to the PieceName
function generateKeyPiecesTable(unitID,piecefunction)

returnTable={}
piecesTable=Spring.GetUnitPieceList(unitID)

	if piecesTable ~= nil then
		for i=1,#piecesTable,1 do
		returnTable[piecesTable[i]]=piecefunction(piecesTable[i])

		end

	end
return returnTable
end

-->generates a Pieces List 
function generatepiecesTableAndArrayCode(unitID)
Spring.Echo("--PIECESLIST::BEGIN  |>----------------------------")
Spring.Echo("--PIECES")
Spring.Echo("piecesTable={}")
piecesTable={}
piecesTable=Spring.GetUnitPieceList(unitID)
--Spring.Echo("local piecesTable={}")
	if piecesTable ~= nil then
		for i=1,#piecesTable,1 do
		workingString=piecesTable[i]
		Spring.Echo("piecesTable[#piecesTable+1]= piece(\""..piecesTable[i].."\")")

		end


	end

Spring.Echo("PIECESLIST::END   |>-----------------------------")
end



--> Transfers a World Position into Unit Space
function worldPosToLocPos(owpX,owpY,owpZ,wpX,wpY,wpZ)
return wpX-owpX,wpY-owpY ,wpZ-owpZ
end

--> Flashes a Piece for debug purposes
function flashPiece(pname,time,rate)
r=rate
t=time or 1000
if not rate then r=50 end

	for i=0,time,2*r do
	Sleep(r)
	Show(pname)
	Sleep(r)
	Hide(pname)
	end


end
--> Hides a PiecesTable, 
function hideT(tablename,lowLimit,upLimit,delay)
if lowLimit and upLimit then
	for i=upLimit,lowLimit, -1 do
	Hide(tablename[i])
	if delay then Sleep(delay) end
	end

else
	for i=1,table.getn(tablename), 1 do
	Hide(tablename[i])
	end
end
end

-->Shows a Pieces Table
function showT(tablename,lowLimit,upLimit,delay)
	if lowLimit and upLimit then
		for i=lowLimit,upLimit, 1 do
		Show(tablename[i])
		if delay then Sleep(delay) end
		end

	else
		for i=1,table.getn(tablename), 1 do
		Show(tablename[i])
		end
	end
end



--> This function process result of Spring.PathRequest() to say whether target is reachable or not
function IsTargetReachable (moveID, ox,oy,oz,tx,ty,tz,radius)
        local result,lastcoordinate, waypoints
        local path = Spring.RequestPath( moveID,ox,oy,oz,tx,ty,tz, radius)
        if path then
                local waypoint = path:GetPathWayPoints() --get crude waypoint (low chance to hit a 10x10 box). NOTE; if waypoint don't hit the 'dot' is make reachable build queue look like really far away to the GetWorkFor() function.
                local finalCoord = waypoint[#waypoint]
                if finalCoord then --unknown why sometimes NIL
                        local dx, dz = finalCoord[1]-tx, finalCoord[3]-tz
                        local dist = math.sqrt(dx*dx + dz*dz)
                        if dist <= radius+20 then --is within radius?
                                result = true
                                lastcoordinate = finalCoord
                                waypoints = waypoint
                        else
                                result = false
                                lastcoordinate = finalCoord
                                waypoints = waypoint
                        end
                end
        else
                result = true
                lastcoordinate = nil
                waypoints = nil
        end
        return result, lastcoordinate, waypoints
end


--> takes a given position and the dir of a surface and calculates the vector by which the vector is reflectd,
--if fall in angle == escape angle
function mirrorAngle(nX,nY,nZ,dirX,dirY,dirZ)
max=math.max(dirX,math.max(dirY,dirZ))
dirX,dirY,dirZ=dirX/max,dirY/max,dirZ/max
max=math.max(nX,math.max(nY,nZ))
nX,nY,nZ=nX/max,nY/max,nZ/max


a=math.atan2(nY,nZ)--alpha	x_axis
b=math.atan2(nX,nY)--beta	z_axis

ca=math.cos(a)
cma=math.cos(-1*a)
ncma=cma*-1
sa=math.sin(a)
sma=math.sin(-1*a)
nsma=sma*-1

cb=math.cos(b)
cmb=math.cos(-1*b)
ncmb=cmb*-1
sb=math.sin(b)
smb=math.sin(-1*b)
nsmb=smb*-1
-- -c(a)*c(-a)+s(a)*s(-a)																		|-c(a)-s(-a)+ s(a)*c(-a)																	|0				|0
-- c(-b)*[(c(b)*s(a)*c(-a)+c(b)*c(a)*s(-a))]+-s(-b)*[(-s(a)*s(b)*c(-a))+(-s(b)*c(a)*s(-a))]		|c(-b)*[c(b)*s(a)*-s(-a) + c(b)*c(a)*c(-a) ]+-s(-b)*[-s(a)*s(b)*-s(-a)+(-s(b)*c(a)*c(-a)]	|-s(b)*s(-b)	|0
-- s(-b)*[c(b)*s(a)*c(-a)+c(b)*c(a)*s(-a)	]+ c(-b) *[ -s(a)*s(b)*c(-a) + (-s(b)*c(a)*s(-a)]	|s(-b)*[c(b)*s(a)*-s(-a) + c(b)*c(a)*c(-a) ]+c(-b)*[-s(a)*s(b)*-s(-a)+(-s(b)*c(a)*c(-a)]	|-c(b)*c(-b)	|0
			
dirX=dirX*ncma*cma+sa*sma+dirY*ncma*-1*sma+ sa*cma		
dirY=dirX*cmb*((cb*sa*cma+cb*ca*sma))+-1*smb*((-1*sa*sb*cma)+(-1*sb*ca*sma))		+dirY*cmb*(cb*sa*-1*sma + cb*ca*cma )+ (-1*smb*-1*sa*sb*-1*sma+-1*sb*ca*cma)	+dirZ*-1*sb*smb
dirZ=dirX*smb*(cb*sa*cma+cb*ca*sma	)+ cmb *( -1*sa*sb*cma + (-1*sb*ca*sma))	+dirY*smb*(cb*sa*-1*sma + cb*ca*cma +cmb*-1*sa*sb*-1*sma+(-1*sb*ca*cma))	+dirZ* -1*cb*cmb

return dirX,dirY,dirZ
end

-->RotationMatrice for allready Normalized Values
function NDrehMatrix(x,z,rad)

	   sinus=math.sin(rad)
	   cosinus= math.cos(rad)
					
	return x*cosinus + z*-sinus, x*sinus  + z*cosinus
end


--forceFunctionTable Example
function hitByExplosionAtCenter(objX,objY,objZ,worldX,worldY,worldZ,objectname,mass,dirX,dirY,dirZ)
objX,objY,objZ=objX-worldX,objY-worldY,objZ-worldZ
distanceToCenter =(objX^2+objY^2+objZ^2)^0.5
blastRadius=350
Force=4000000000000 
factor=blastRadius/(2^distanceToCenter)
	if distanceToCenter > blastRadius then factor=0 end
normalIsDasNicht=math.max(math.abs(objX),math.max(math.abs(objY),math.abs(objZ)))

objX,objY,objZ=objX/normalIsDasNicht,objY/normalIsDasNicht,objZ/normalIsDasNicht
			--density of Air in kg/m^3 -- area
airdrag=0.5*1.1455*((normalIsDasNicht*factor*Force)/mass)^2*15*0.47

	if math.abs(objX) == 1 then
	return objX*factor*Force-airdrag,objY*factor*Force, objZ*factor*Force
		elseif math.abs(objY)==1 then

		return objX*factor*Force,objY*factor*Forceairdrag, objZ*factor*Force
			else

			return objX*factor*Force,objY*factor*Force, objZ*factor*Forceairdrag
			end
end
--> a Pseudo Physix Engien in Lua, very expensive, dont use extensive	--> forceHead(objX,objY,objZ,worldX,worldY,worldZ,objectname,mass)
function PseudoPhysix(piecename,pearthTablePiece, nrOfCollissions, forceFunctionTable)


					speed=math.random(1,4)
					rand=math.random(10,89)
					Turn(piecename,x_axis,math.rad(rand),speed)
					dir=math.random(-90,90)
					speed=math.random(1,3)
					Turn(piecename,y_axis,math.rad(dir),speed)

		
	--SpeedUps
	local spGPos=Spring.GetUnitPiecePosDir
	local spGGH=Spring.GetGroundHeight
	local spGN=Spring.GetGroundNormal
	local mirAng=mirrorAngle		
	local spGUPP=Spring.GetUnitPiecePosition
	local spGUP=Spring.GetUnitPosition
	local ffT=forceFunctionTable			
	posX,posY,posZ,dirX,dirY,dirZ=spGPos(unitID,pearthTablePiece)
	ForceX,ForceY,ForceZ=0,0,0
	oposX,oposY,oposZ=posX,posY,posZ
	

	mass=600
	simStep=75
	gravity=-1*(Game.gravity) -- in Elmo/s^2  --> Erdbeschleunigung 
	
	--tV=-1* 
	terminalVelocity=-1*(math.abs((2*mass*gravity))^0.5	)
	ForceGravity=-1*(mass*Game.gravity) --    kgE/ms^2
	
	GH=spGGH(posX,posZ)		--GroundHeight
	if oposY < GH then oposY=GH end
	
	

	VelocityX,VelocityY,VelocityZ=0,0,0
	factor =(1/1000)*simStep
	
	
	boolRestless=true
	
		while boolRestless==true  do
		
		-- reset
		ForceX,ForceY,ForceZ=0,0,0
		
		-- update
		posX,posY,posZ,dirX,dirY,dirZ=spGPos(unitID,pearthTablePiece)	
		_,_,_,dirX,dirY,dirZ=spGPos(unitID,piecename)	
		
		--normalizing
		normalizer=math.max(math.max(dirX,dirY),dirZ)
		if normalizer == nil or normalizer== 0 then normalizer=0.001 end
		dirX,dirY,dirZ=dirX/normalizer,dirY/normalizer,dirZ/normalizer
		
		-- applying gravity and forces 
		ForceY=ForceGravity
		
		if ffT ~= nil then --> forceHead(objX,objY,objZ,oDirX,oDirY,oDirZ,objectname,dx,dy,dz)
		bx,by,bz=Spring.spGUP(unitID)
		dx,dy,dz=spGUPP(unitID,piecename)
		dmax=math.sqrt(dx^2+dy^2+dz^2)
		dx,dy,dz=dx/dmax,dy/dmax,dz/dmax
		
		for i=1, #ffT, 1 do	
		f2AddX,f2AddY,f2AddZ=ffT[i](posX,posY,posZ,bx,by,bz,piecename,mass,dx,dy,dz)
		ForceX=ForceX+f2AddX
		ForceY=ForceY+f2AddY
		ForceZ=ForceZ+f2AddZ
		end
		
		
		end

		GH=spGGH(posX+VelocityX+(ForceX/mass)*factor,	posZ+VelocityZ+(ForceZ/mass)*factor)
		boolCollide=false
		
		
		--GROUNDcollission
	
		if  (posY  -GH-15) < 0 then
		boolCollide=true
		nrOfCollissions=nrOfCollissions-1
			
			
				total=math.abs(VelocityX)+math.abs(VelocityY)+math.abs(VelocityZ)
				--get Ground Normals
				
				nX,nY,nZ=spGN(posX,posZ)
				max=math.max(nX,math.max(nY,nZ))
				_,tnY,_=nX/max,nY/max,nZ/max
				
				--if still enough enough Force or stored kinetic energy
				if total > 145.5  or nrOfCollissions > 0 or tnY < 0.5 then
				else
					--PhysixEndCase
					boolRestless=false
				end
				
			
				VelocityX,VelocityY,VelocityZ=0,0,0
				

				--could do the whole torque, but this prototype has fullfilled its purpose
			--	up=math.max(math.max((total/mass)%5,4),1)+1
				
				dirX,dirY,dirZ=mirAng(nX,nY,nZ,dirX,dirY,dirZ)
				speed=math.random(0.55,7)
				Turn(piecename,x_axis,dirX,speed)
				speed=math.random(0.55,6)
				Turn(piecename,y_axis,dirY,speed)
				speed=math.random(0.55,5)
				Turn(piecename,z_axis,dirZ,speed)
			
				
				
				--we have the original force * constant inverted - Gravity and Ground channcel each other out
				RepulsionForceTotal=((math.abs(ForceY)+math.abs(ForceZ)+math.abs(ForceX))*-0.65)
				ForceY=ForceY+((dirY*RepulsionForceTotal))					
				
				ForceX=ForceX+((dirX*RepulsionForceTotal))
				ForceZ=ForceZ+((dirZ*RepulsionForceTotal))
				VelocityY=math.max(VelocityY+((ForceY/mass)*factor),terminalVelocity*factor)
				
		else
		
		--FreeFall		
		
		VelocityY=math.max(VelocityY+((ForceY/mass)*factor),terminalVelocity*factor)
		end
		

		
		VelocityX=math.abs(VelocityX+(ForceX/mass)*factor)
		VelocityZ=math.abs(VelocityZ+(ForceZ/mass)*factor)
		
		--Extract the Direction from the Force
		xSig=ForceX/math.max(math.abs(ForceX),0.000001)
		ySig=ForceY/math.max(math.abs(ForceY),0.000001)
		zSig=ForceZ/math.max(math.abs(ForceZ),0.000001)
		
			-- FuturePositions
			fX,fY,fZ=worldPosToLocPos(oposX,oposY,oposZ,posX+math.abs(VelocityX)*xSig, posY + math.abs(VelocityY)*ySig  ,posZ+math.abs(VelocityZ)*zSig)
		
		if  boolCollide== true or boolRestless== false or (fY -GH -12 < 0) then fY= -1*oposY+GH end
			--Debugdatadrop

		--	Spring.Echo("ySig",ySig.."	Physix::ComendBonker::fY",fY.."VelocityY::",VelocityY .."    	ForceY::",ForceY .."    	POSVAL:", posY + VelocityY*ySig)

			Move(pearthTablePiece,x_axis,fX,VelocityX*1000/simStep+0.000000001)
			Move(pearthTablePiece,y_axis,fY,VelocityY*1000/simStep)
			Move(pearthTablePiece,z_axis,fZ,VelocityZ*1000/simStep+0.000000001)
			
		
		
		Sleep(simStep)
		end
	

	
end

function createMass(mass,px,py,pz,vx,vy,vz,fx,fy,fz)
mass={m=mass,pos={x=px,y=py,z=pz},vel={x=vx,y=vy,z=vz},force={x=fx,y=fy,z=fz}}
return mass
end

function debugDisplayPieceChain(Tables)
	for i=1, #Tables, 1 do
	x,y,z,_,_,_=Spring.GetUnitPiecePosDir(Tables[i])
	Spring.SpawnCEG("redlight",x,y+10,z,0,1,0,50,0)
	end

end

function iniT(size,val)
T={}
	for i=1,size do
	T[i]=val
	end
return T
end

function vectorMinus(v1,v2)
return{x=v1.x-v2.x,y=v1.y-v2.y,z=v1.z-v2.z}
end

local countConstAnt=0
function vMul(v1,value)
countConstAnt=countConstAnt+1
--if not value or type(value)~='number' and #value == 0  then Spring.Echo("JW::RopePhysix::"..countConstAnt)end 

	if value and  type(value)=='number' then --Skalar
	return {x = v1.x*value,
			y=v1.y*value,
			z=v1.z*value}
		else		--return vector
		Spring.Echo("JW:ToolKit:Vmul"..countConstAnt)
		return {x = v1.x*value.x,y=	v1.y*value.y, z=	v1.z*value.z}
		end
end

function vectorLenght(v1,v2)
v=vectorMinus(v1,v2)
return math.sqrt(v.x*v.x +v.y*v.y +v.z*v.z)
end

function VdivV(v1, val)
if not val.x then
return {x=v1.x/val,y=v1.y/val,z=v1.z/val}
else
return {x=v1.x/val.x,y=v1.y/val.y,z=v1.z/val.y}
end
end

function VaddV(v1,val)
return {x= v1.x+val.x, y= v1.y+val.y,z=v1.z+val.z}
end

function Vsub(v1,val)
return {x=v1.x-val,y=v1.y-val,z=v1.z-val}
end

function VsubV(v1,v2)
return {x=v1.x-v2.x,y=v1.y-v2.y,z=v1.z-v2.z}
end

function applyForce(force,force2)
return VaddV(force,force2)
end

function normalizeVector(v)
l=math.sqrt(v.x*v.x +v.y*v.y +v.z*v.z)
return {x=v.x/l ,y=v.y/l,z=v.z/l}
end

function solveSpring(s, sucessor, frictionConstant)
  springVector =vectorMinus(s.mass1.pos,sucessor.mass1.pos)      -- Vector Between The Two Masses
         
         r = vectorLenght(springVector)                -- Distance Between The Two Masses
 
        force={x=0,y=0,z=0}                         -- Force Initially Has A Zero Value
         
        if r ~= 0 then                         -- To Avoid A Division By Zero... Check If r Is Zero  
            force = VaddV(vMul(vMul(VdivV(springVector, r),-1),((r - s.length) * s.springConstant)),force)
		end
		    force = VaddV(force, vMul(VsubV(s.mass1.vel, sucessor.mass1.vel),-1*frictionConstant))     -- The Friction Force Is Added To The force
                                    -- With This Addition We Obtain The Net Force Of The Spring
	s.mass1=VaddV(s.mass1,force)			-- Force Is Applied To mass1
	sucessor.mass1=VaddV(sucessor.mass1,vMul(force,-1))	-- The Opposite Of Force Is Applied To mass2	
return s,sucessor	
end

function stringOfLength(char,length)
strings=""
for i=1,length do strings=strings..char end
return strings
end

function rEchoTable(T,layer)
l=layer or 0
	if T then
		if type(T)=='table' then
		Spring.Echo("+"..(stringOfLength("_",l)).."___ RTable ")
			for k,v in pairs(T) do
			rEchoTable(T[k],l+1)
			end
		Spring.Echo((stringOfLength("_",l)))
		else
			Concated=stringOfLength(" ",math.max(1,l)-1).."|"
			
			typus= type(T)
			if typus == "number" or typus == "string" then
			Spring.Echo(Concated..T)
				elseif typus=="boolean" then
				Spring.Echo(Concated.."boolean"..((T==true) and "True"))
					else
					Spring.Echo(Concated.."function")
					end
		end
	
	end
end

function echoTable(T,boolAssertTable)
lboolAssertTable=boolAssertTable or false

if T.name then 
Spring.Echo("============================= "..T.name.." ======================================")
else
Spring.Echo("============================= EchoTable =========================================")
end
for k,v in pairs(T) do
typek=type(k)
typev=type(v)
typek=typek~="table" and typek ~="function"
typev=typev~="table" and typev ~="function"
	if typek and typev then
	Spring.Echo("  "..k.."  	--->  	"..v .."   ->  	[ "..(assert(v)).." ]  ")
	end
end

if lboolAssertTable ==true then
	for k,v in pairs(T) do
	assert(v)
	end
end


Spring.Echo("================================================================================")

end


function checkYourself()
return GG.SniperRope[unitID] or false
end

function Limit(val,min,max)
if val < min then return min end
if val > max then return max end
return val 
end

-- This code is a adapted Version of the NeHe-Rope Tutorial. All Respect towards those guys.
-- RopePieceTable by Convention contains (SegmentBegin)----(SegmentEnd)(SegmentBegin)----(SegmentEnd) 
-- RopeConnectionPiece -->Piece,ContainsMass,ColRadius |
-- LoadPiece --> Piece,Contains Mass, ColRadius | 
-- ForceFunction --> forceHead(objX,objY,objZ,worldX,worldY,worldZ,objectname,mass)

										 
function PseudoRopePhysix(RopePieceTable,RopeConnectionT,LoadPieceT, Ropelength, forceFunctionTable,SpringConstant,boolDebug)
	
	--SpeedUps
assert(RopePieceTable 		,"RopePieceTable not provided")			
assert(RopeConnectionT 		,"RopeConnectionT not provided")			
assert(LoadPieceT 			,"LoadPieceT not provided")			
assert(Ropelength 			,"Ropelength not provided")			
assert(forceFunctionTable 	,"forceFunctionTable not provided")			
assert(SpringConstant 		,"SpringConstant not provided")					

	local spGPos=Spring.GetUnitPiecePosDir
	local spGGH=Spring.GetGroundHeight
	local spGN=Spring.GetGroundNormal	
	local spGUPP=Spring.GetUnitPiecePosition
	local spGUP=Spring.GetUnitPosition
	local ffT=forceFunctionTable			
	local    groundHeight  = function(piece) 
					x,y,z,_,_,_=Spring.GetUnitPiecePosDir(unitID,piece)
					return Spring.GetGroundHeight(x,z) end     
					-- Each Particle Has A Weight Of 50 Grams


	--SIMCONSTANTS   -tweak them
local	SpringConstant=SpringConstant or 100000.0 

local    SpringInnerFriction= 0.2
local 	 RopeMass					= 0.1
local 	 groundRepulsionConstant 	= 100                     -- A Constant To Represent How Much The Ground Shall Repel The Masses
local    groundFrictionConstant 	= 0.35                     -- A Constant Of Friction Applied To Masses By The Ground
local    groundAbsorptionConstant	= 3               -- A Constant Of Absorption Friction Applied To Masses By The Ground
local    airFrictionConstant  		= 0.5                        

	--INIT

ForcesTable={}

rcx,rcy,rcz=Spring.GetUnitPiecePosDir(unitID,RopeConnectionT.Piece)
RopeConnection={
		  Pos={x=rcx,y=rcy,z=rcz},
		  vel={x=0,y=0,z=0},
		  mass=1500, --unrealistic, yet you cant reach escape velocity 
		  Radius=RopeConnectionT.ColRadius
		  }
		  
local ObjT={}
	--contains the startpos, as the sensory piece
	for i=1,#RopePieceTable, 1 do
	px,py,pz,pdx,pdy,pdz=Spring.GetUnitPiecePosDir(unitID,RopePieceTable[i])
		
	ObjT[#ObjT+1]={ 
				piecename=RopePieceTable[i],
				posdir={x,y,z,xd,yd,zd=px,py,pz,pdx,pdy,pdz},
				mass1={mass= createMass(RopeMass/2,px,py,pz,0,0,0,0,0,0)},
				vel={x=0,y=0,z=0},
				PrevPiece=(math.max(1,i-1)),
				NextPiece=math.min(i+1,#RopePieceTable),
				length=Ropelength,
				springConstant=SpringConstant
	}
	end
	--The pulled Object
	px,py,pz,pdx,pdy,pdz=Spring.GetUnitPiecePosDir(unitID,LoadPieceT.Piece)
	ObjT[#ObjT+1]={ 
				piecename=LoadPieceT.Piece,
				posdir={x,y,z,xd,yd,zd=px,py,pz,pdx,pdy,pdz},
				mass1={mass= createMass(LoadPieceT.Mass,px,py,pz,0,0,0,0,0,0)},
				vel={x=0,y=0,z=0},
				PrevPiece=(#ObjT),
				length=Ropelength,
				springConstant=SpringConstant
	}
	--/INIT
	
	
	local simStep=75

	
	
	while checkYourself()  do
			--init and reset
			for i=1,#ObjT,1 do
			ObjT[i].mass1.force={x=0,y=0,z=0}
			end

		--applyForces
		
		for j=1,#ffT,1 do 
		if ffT.pieceList then
			for k in ipairs(fft.piecelist) do
				if not ffT[j].geometryfunction or ffT[j].geometryfunction(ObjT[k].posdir.x,ObjT[k].posdir.y,ObjT[k].posdir.z) == true then
				ObjT[k].mass1.force=VaddV(ObjT[k].mass1.force, vMul(ffT[j].acceleration,ObjT[k].mass1.mass))		  
				end
			end
		else
			for i=1,#ObjT, 1 do
			ObjT[i].mass1.force=VaddV(ObjT[i].mass1.force, vMul(ffT[j].acceleration,ObjT[i].mass1.mass))		
			end
		end
		end	
		
		--Lets SpringSimulation
		for i=1,#ObjT, 2 do
		ObjT[i],ObjT[Limit(i+1,1,#ObjT)] =solveSpring(ObjT[i],ObjT[Limit(i+1,1,#ObjT)],SpringInnerFriction)	
		end		
		
		
	  --solve Objforces
	for i=1,#ObjT, 2 do              -- Start A Loop To Apply Forces Which Are Common For All Masses
		local Succesor=Limit(i,1,ObjT)
		
		ObjT[i].mass1.force=VaddV(ObjT[i].mass1.force,vMul(gravitation,ObjT[i].mass1.mass))
	
		--    masses[a]->applyForce(gravitation * masses[a]->m);    -- The Gravitational Force
        -- The air friction
		ObjT[i].mass1.force=VaddV(ObjT[i].mass1.force, vMul(ObjT[i].vel, airFrictionConstant*-1))
		
    --    masses[a]->applyForce(-masses[a]->vel * airFrictionConstant);
 
 
		if 	ObjT[i].mass1.pos.y > groundHeight(ObjT[i].piecename)  then
            -- Forces From The Ground Are Applied If A Mass Collides With The Ground
        
             v1={x=0,y=0,z=0}                 -- A Temporary Vector3D
             v2={x=0,y=0,z=0}                 -- A Temporary Vector3D
			
			v1=ObjT[i].mass1.vel
            v2 = ObjT[Succesor].mass2.vel              -- Get The Velocity
			
			v1.y=0
			v2.y=0
                             -- Omit The Velocity Component In Y-Direction
 
            -- The Velocity In Y-Direction Is Omited Because We Will Apply A Friction Force To Create
            -- A Sliding Effect. Sliding Is Parallel To The Ground. Velocity In Y-Direction Will Be Used
            -- In The Absorption Effect.
 
            -- Ground Friction Force Is Applied   
			ObjT[i].mass1.force=VaddV(ObjT[i].mass1.force,vMul(vMul(v1,-1),groundFrictionConstant))
			ObjT[Succesor].mass1.force=VaddV(ObjT[Succesor].mass1.force,vMul(vMul(v2,-1),groundFrictionConstant))			
         
 
            v1 =ObjT[i].mass1.vel              -- Get The Velocity
            v2 =ObjT[Succesor].mass1.vel              -- Get The Velocity
			
			v1.x,v1.z=0,0
			v2.x,v2.z=0,0
   
            -- Above, We Obtained A Velocity Which Is Vertical To The Ground And It Will Be Used In
            -- The Absorption Force
 
            if (v1.y < 0) then              
            	ObjT[i].mass1.force=VaddV(ObjT[i].mass1.force, vMul(vMul(vMul(v1,-1),groundAbsorptionConstant)))  
			end
			
			if ( v2.y < 0) then
			 	ObjT[Succesor].mass1.force=VaddV(ObjT[Succesor].mass1.force, vMul(vMul(vMul(v2,-1),groundAbsorptionConstant)))
			end
            
			x,y,z,_,_,_=Spring.GetUnitPiecePosDir(unitID,ObjT[i].piecename)
			vx,vy,vz=Spring.GetGroundNormal(x,z)
			else
				-- The Ground Shall Repel A Mass Like A Spring.
				-- By "Vector3D(0, groundRepulsionConstant, 0)" We Create A Vector In The Plane Normal Direction
				-- With A Magnitude Of groundRepulsionConstant.
				-- By (groundHeight - masses[a]->pos.y) We Repel A Mass As Much As It Crashes Into The Ground.
				gh=groundHeight(	ObjT[i].piecename)
				f1=vMul(vMul({x=vx,y=vy,z=vz},groundRepulsionConstant),gh-ObjT[i].mass1.pos.y)
				f2=vMul(vMul({x=vx,y=vy,z=vz},groundRepulsionConstant),gh-ObjT[Succesor].mass1.pos.y)
				
				ObjT[Succesor].mass1.force=VaddV(ObjT[Succesor].mass1.force,f2)		
				ObjT[i].mass1.force=VaddV(ObjT[i].mass1.force,f1)		      -- The Ground Repulsion Force Is Applied
			end
	end	   
		  --simulate 
			for i=1, #ObjT, 1 do
			--vel += (force/mass)* dt
			ObjT[i].mass1.vel= VaddV(ObjT[i].mass1.vel,vMul( VdivV(ObjT[i].mass1.force,ObjT[i].mass1.m),timeInMS))
			ObjT[Succesor].mass1.vel= VaddV(ObjT[Succesor].mass1.vel,vMul( VdivV(ObjT[Succesor].mass1.force,ObjT[Succesor].mass1.m),timeInMS))
			ObjT[i].mass1.pos=VaddV(ObjT[i].mass1.pos, Vsub(ObjT[i].mass1.vel,timeInMS))
			ObjT[Succesor].mass1.pos=VaddV(ObjT[Succesor].mass1.pos, Vsub(ObjT[Succesor].mass1.vel,timeInMS))
			end
		--TranslatePieces to new Positions
		
		Sleep(simStep)
		if boolDebug then
		debugDisplayPieceChain(RopePieceTable)
		end
	end
	

	

end

--add Operators to unitscript

function eraNonArgMul(A,B)
if A== "0" or B=="0" or A== "" or B=="" then return "" 
else return "("..A.."*"..B..")" end
end

function eraNonArgAdd(A,B)
if A== "0"  or A== "" and B ~= "0" and B ~="" then return B end
if B== "0"  or B== "" and A ~= "0" and A ~="" then return A end
return A.."+"..B

end
--> used to calc shadder matrixes in lua.. fill in, calc, optimize, print out
function matrixLab()
mA={
[1]="cos(z)",	[2]="-sin(z)",	[3]="0",
[4]="sin(z)",	[5]="cos(z)",	[6]="0",
[7]="0",	[8]="0",	[9]="1",	
	}
mB={
[1]="-1",	[2]="0",		[3]="0",
[4]="0",	[5]="cos(x)",	[6]="-sin(x)",
[7]="0",	[8]="sin(x)",	[9]="cos(x)",	
}
mC={
[1]="cos(y)",	[2]="0",	[3]="sin(y)",
[4]="0",		[5]="1",	[6]="0",
[7]="-sin(y)",	[8]="0",	[9]="cos(y)",	
	}


	mD=MatrixBuilder3x3(MatrixBuilder3x3(mA,mB),mC)
	echoTable(mD)


end

function mirrorMatriceXAxis(x,y,z)
return 360-x,y,z*-1																																																																																																																																																																																																																																																																																	

--x=	((-1*math.cos(z))*math.cos(y))+((-1*math.sin(z)*-1*math.sin(x))*-1*math.sin(y)) 	*x + 	((-1*math.sin(z)*math.cos(x)) )		*y+   	((-1*math.cos(z))*math.sin(y))+((-1*math.sin(z)*-1*math.sin(x))*math.cos(y))   *z
--y= 	((-1*math.sin(z))*math.cos(y))+((math.cos(z)*-1*math.sin(x))*-1*math.sin(y))   *x +	     ((math.cos(z)*math.cos(x)) )	*y+   ((-1*math.sin(z))*math.sin(y))+((math.cos(z)*-1*math.sin(x))*math.cos(y))   *z
--z=	((math.cos(x))*-1*math.sin(y)) 								*x + ((math.sin(x)) )   			*y+    	((*math.cos(x))*math.cos(y))  *z
--return x,y,z
end

function MatrixBuilder3x3(A, B)
return {
[1]=eraNonArgAdd(eraNonArgAdd(eraNonArgMul(A[1],B[1]),eraNonArgMul(A[2],B[4])),eraNonArgMul(A[3],B[7])),
[2]=eraNonArgAdd(eraNonArgAdd(eraNonArgMul(A[1],B[2]),eraNonArgMul(A[2],B[5])),eraNonArgMul(A[3],B[8])),
[3]=eraNonArgAdd(eraNonArgAdd(eraNonArgMul(A[1],B[3]),eraNonArgMul(A[2],B[6])),eraNonArgMul(A[3],B[9])),
[4]=eraNonArgAdd(eraNonArgAdd(eraNonArgMul(A[4],B[1]),eraNonArgMul(A[5],B[4])),eraNonArgMul(A[6],B[7])),
[5]=eraNonArgAdd(eraNonArgAdd(eraNonArgMul(A[4],B[2]),eraNonArgMul(A[5],B[5])),eraNonArgMul(A[6],B[8])),
[6]=eraNonArgAdd(eraNonArgAdd(eraNonArgMul(A[4],B[3]),eraNonArgMul(A[5],B[6])),eraNonArgMul(A[6],B[9])),
[7]=eraNonArgAdd(eraNonArgAdd(eraNonArgMul(A[7],B[1]),eraNonArgMul(A[8],B[4])),eraNonArgMul(A[9],B[7])),
[8]=eraNonArgAdd(eraNonArgAdd(eraNonArgMul(A[7],B[2]),eraNonArgMul(A[8],B[5])),eraNonArgMul(A[9],B[8])),
[9]=eraNonArgAdd(eraNonArgAdd(eraNonArgMul(A[7],B[3]),eraNonArgMul(A[8],B[6])),eraNonArgMul(A[9],B[9])),
}
end

--> Move all Elements of a Table to Zero
function zeroT(t)
for i=1, #t, 1 do
Move(t[i],y_axis,0,0)
Move(t[i],z_axis,0,0)
Move(t[i],z_axis,0,0)
end
end

--> Turn a Table towards local T
function turnTable(t, axis, deg,speed,boolInstantUpdate)
	if boolInstantUpdate then
		for i=1,#t,1 do
		Turn(t[i],axis,math.rad(deg),0,true)
		end
		return
	end

	if not speed or speed==0 then
		for i=1,#t,1 do
		Turn(t[i],axis,math.rad(deg),0)
		end
	else
		for i=1,#t,1 do
		Turn(t[i],axis,math.rad(deg),speed)
		end
	end
return
end

function turnTableRand(t, taxis, uparg, downarg,speed,boolInstantUpdate)
axis=taxis or 2  --y_axis as default
down=downarg or math.random(-50,0)
up=uparg or math.random(0,50)
	if down > up then down=down*-1-1 end
	
	if boolInstantUpdate then
		for i=1,#t,1 do
		Turn(t[i],axis,math.rad(math.random(down,up)),0,true)
		end
		return
	end

	if not speed or speed==0 then
		for i=1,#t,1 do
		Turn(t[i],axis,math.rad(math.random(down,up)),0)
		end
	else
		for i=1,#t,1 do
		Turn(t[i],axis,math.rad(math.random(down,up)),speed)
		end
	end
return
end

function spawnCegAtPiece(unitID,pieceId,cegname,offset)
boolAdd=offset or 10


if not unitID then error("ToolKit::Not enough arguments to spawnCEGatUnit") end
if  not pieceId then  error("ToolKit::Not enough arguments to spawnCEGatUnit") end
if not cegname then error("ToolKit::Not enough arguments to spawnCEGatUnit") end
x,y,z=Spring.GetUnitPiecePosDir(unitID,pieceId)

if y then
	y=y+boolAdd
	Spring.SpawnCEG(cegname,x,y,z,0,1,0,0,0)
end
end

--> Play a soundfile only by unittype
function PlaySoundByUnitType(unitdef, soundfile,loudness, time, nrOfUnitsParallel,predelay)
if predelay then Sleep(predelay) end

loud=loudness or 1
if loud==0 then loud= 1 end

if GG.UnitDefSoundLock == nil then  GG.UnitDefSoundLock={} end
if GG.UnitDefSoundLock[unitdef] == nil then  GG.UnitDefSoundLock[unitdef]=0 end

	if  GG.UnitDefSoundLock[unitdef] < nrOfUnitsParallel then
	GG.UnitDefSoundLock[unitdef]=GG.UnitDefSoundLock[unitdef]+1
	Spring.PlaySoundFile(soundfile,loud)
		if time <= 0 then time =2500 end
	Sleep(time)
	GG.UnitDefSoundLock[unitdef]=GG.UnitDefSoundLock[unitdef]-1
	return true
	end
return false
end

--This Takes any LanguageString and 'translates' it meaning it replaces stringparts  with the Sound
--Please take note that this should not completely replace any selfspoken sound - its a addition
--SoundPerson is a Function that allows to convay additional params into the sound-
--e.g. Out of Breath, Angry, tired, sad, by changing loudness and choosen soundsnippet
--its call signature is SoundPerson(translatedSoundSnippet, position in sentence, translatedTable)
function speakItalian(LanguageTable, SoundTable, Text, ScreenPos, StandardLoud, LoudRange, SoundPerson)
--make a text subtitles
gl.Text(Text, ScreenPos.x,ScreenPos.y, ScreenPos.z, 12)

--translate the Text via the language Table
local lplaySoundFile=Spring.PlaySoundFile
local translatedTable={}
local lSoundPerson=SoundPerson or nil

	for i = 1, #Text do
	   c = str:sub(i,i)
	   translatedTable[i]=LanguageTable[c] or " "
	end
	
	if lSoundPerson then
		for it=1,#translatedTable  do
		choosenSound,loudness=lSoundPerson(translatedTable[it],it,translatedTable)
			if SoundTable[choosenSound] then
			lplaySoundFile(SoundTable[choosenSound], loudness)
			end
		end
	else
	lplaySoundFile(SoundTable[choosenSound], StandardLoud+math.random(LoudRange*0.1,LoudRange))
	end
end

function getLowestPointOfSet(Table,axis)
index=1
y=math.huge
if axis=="y_axis" then
	for i=1,#Table do
			if Table[i].y < y then 
			y=Table[i].y
			index=i
			end
		
	end
end
if axis=="z_axis" then
	for i=1,#Table do
			if Table[i].z < y then 
			y=Table[i].z
			index=i
			end
		
	end
end
if axis=="x_axis" then
	for i=1,#Table do
			if Table[i].x < y then 
			y=Table[i].x
			index=i
			end
		
	end
end

			
return Table[index].x,Table[index].y,Table[index].z,index

end

function getHighestPointOfSet(Table,axis)
index=1
y=math.huge*-1
if axis=="y_axis" then
	for i=1,#Table do
			if Table[i].y > y then 
			y=Table[i].y
			index=i
			end
		
	end
end
if axis=="z_axis" then
	for i=1,#Table do
			if Table[i].z > y then 
			y=Table[i].z
			index=i
			end
		
	end
end
if axis=="x_axis" then
	for i=1,#Table do
			if Table[i].x > y then 
			y=Table[i].x
			index=i
			end
		
	end
end

			
return Table[index].x,Table[index].y,Table[index].z,index
end

function piec2Point(piecesTable)
	if #piecesTable > 7 then
	local spGetUnitPiecePos=Spring.GetUnitPiecePosDir
	reTab={}
		for i=1,#piecesTable do
		reTab[i]={}
		reTab[i].x,reTab[i].y,reTab[i].z=spGetUnitPiecePos(unitID,piecesTable[i])
		reTab[i].index=i
		end

	return reTab
		else
		reTab={}
			for i=1,#piecesTable do
			reTab[i]={}
			reTab[i].x,reTab[i].y,reTab[i].z=Spring.GetUnitPiecePosDir(unitID,piecesTable[i])
			reTab[i].index=i
			end

		return reTab
		end
end

--> Turns a Pieces table according to a function provided
function waveATable(Table, axis, func, signum, speed,funcscale,totalscale, boolContra,offset)
boolCounter=boolContra or false
offset=offset or 0
scalar= signum* (totalscale)
nr=#Table
pscale=funcscale/nr
total=0

	for i=1,nr do
	val=scalar*func(offset+i*pscale)
		if boolCounter == true then
		Turn(Table[i],axis,math.rad(total+val),speed)
		total=total+val
		else
		Turn(Table[i],axis,math.rad(val),speed)
		end
	end


end

--> spawn a ceg on the map above ground
function markPosOnMap(x,y,z, colourname)


h=Spring.GetGroundHeight(x,z)
if h > y then y=h end
	for i=1,5 do
	Spring.SpawnCEG(colourname,x,y+10,z,0,1,0,50,0)
	Sleep(200)
	end
end

--> Takes a Table of Locks and locks it if the lock is free 
function TestSetLock(Lock,number)
	if TestLocks(number)==true then 
	Lock[number]=true
	return true
	end 
	return false
end

-->Test a rows of locks 
function TestLocks(Lock, number)
	for i=1,table.getn(Lock) do
		if number ~=i and Lock[i]==true then return false end
	end
return true
end

--> Sets a Lock free
function ReleaseLock(Lock,number)
Lock[number]=false
end

	function filterOutTeam(TableOfUnits,teamid)
	returnTable={}
		for i=1,#TableOfUnits, 1 do
		tid=Spring.GetUnitTeam(TableOfUnits[i])
			if tid ~= teamid then returnTable[#returnTable+1]=TableOfUnits[i] end
		end
	return returnTable
	end
	
	--> Grabs every Unit in a circle, filters out the unitid
	function grabEveryone(unitID,x,z,Range,teamid)
	T={}
	if teamid then
	T=Spring.GetUnitsInCylinder(x,z,Range,teamid)
	else
	T=Spring.GetUnitsInCylinder(x,z,Range)
	end
	
	if T and #T>1 and type(unitID)=='number' then
		table.remove(T,unitID)
		end
	return T
	end 
	
	function filterTableByTable(T,T2,compareFunc)
	reTable={}
		for i=1,#T do
		if compareFunc(T[i],T2)==true then reTable[#reTable+1]=T[i] end
		end
	return reTable
	end
	

	
	function keyTableToTable(T,T2,Keyfunction)
	KeyTable={}
		for i=1,#T do
		KeyTable[T[i]]=Keyfunction(T[i],T2)
		end
	return reTable
	
	end
	
	function insertKeysIntoTable(T,T2)
		for i=1,#T do
			if not T2[T[i]] then
			T2[T[i]]=T[i]	
			end
		end	
	return T2
	end
	
	--itterates over a Keytable, executing a function 
	function foreach(T,fooNction)
	reTable={}
		if type(fooNction)=="string" then
		fooNction=string.load("function(k,v)\n"..fooNction.. "\n end")
		assert(type(fooNction)=="function", "string not a function in foreach(k,v) @ toolKit.lua")
		end
		
		for k,v in pairs(T) do	
		reTable[k]=fooNction(k,v)
		end
		
	return reTable
	end
	
	--itterates over a Keytable, executing a function 
	function elementWise(T,fooNction,ArghT)
	reTable={}
	
	for i=1,#T, 1 do
	reTable[i]=fooNction(T[i],ArghT)		
	end
			
	return reTable
	end
	
	
	function countKey(T)
	it=0
		for k,v in pairs(T) do
		it=it+1
		end
	return it
	end
	
	--> takes a Table, and executes ArgTable/Function,Functions on it
	function process(Table,...)
	T={}
	if Table then T=Table else Spring.Echo("Lua:Toolkit:Process: No Table handed over") return end
	if not arg then Spring.Echo("process has not functions to work on table") return  end
	if type(arg)== "function" then return elementWise(T,arg) end

	
	TempArg={}
	TempFunc={}
	--if not arg then return Table end
	
         for _, f in pairs(arg) do
				if type(f)=="function" then
				T=elementWise(T,f,TempArg)				
				TempArg={}			
			
				else				
				TempArg=f
				end			   
         end
	return T
	end
	
	function accessInOrder(T,...)
	local TC=T
	  for _, f in pairs(arg) do
	  executableString="function(TC) if TC["..f.."] then TC=TC[f] return true,TC else return false,TC end end"
	  f=string.load(executableString)
	  TC,bool=f(TC)
		if bool ==false then return false end
	  
	  end
	return true
	end
	
	-->filtersOutUnitsOfType. Uses a Cache, if handed one to return allready Identified Units
	function filterOutUnitsOfType(Table, UnitTypeTable,Cache)
	if Cache then
	returnTable={}
		for i=1, #Table do
			if Cache[Table[i]] and Cache[Table[i]]==true or not UnitTypeTable[Spring.GetUnitDefID(Table[i])] then
			Cache[Table[i]]=true
			returnTable[#returnTable+1]=Table[i]
			else
			Cache[Table[i]]=false
			
			end
		end
		return returnTable,Cache
		
	else
	returnTable={}
		for i=1, #Table do
			if not UnitTypeTable[Spring.GetUnitDefID(Table[i])] then
			returnTable[#returnTable+1]=Table[i]
			end
		
		end
	return returnTable
	end
	end
	-->filters Out TransportUnits
    function filterOutTransport		(T)
   returnTable={}  
		for i=1,#T do    
		def=Spring.GetUnitDefID(T[i])    
		if false== UnitDefs[def].isTransport		 then 
		returnTable[#returnTable+1]=T[i] end  
		end  
	return returnTable  
	end  

 -->filters Out Immobile Units
	function filterOutImmobile (T)
    returnTable={}  
     for i=1,#T do    
      def=Spring.GetUnitDefID(T[i])    
    	   if false== UnitDefs[def].isImmobile         then 
    	  returnTable[#returnTable+1]=T[i]
    	  end  
       end  
    return returnTable  
     end
--> filters Out Buildings
    function filterOutBuilding       (T)
       returnTable={}  
     for i=1,#T do    
      def=Spring.GetUnitDefID(T[i])    
       if false== UnitDefs[def].isBuilding         then 
      returnTable[#returnTable+1]=T[i] end  
     end  
    return returnTable  
    end
 
--> filters Out Builders
    function filterOutBuilder        (T)
   returnTable={}  
		for i=1,#T do    
		def=Spring.GetUnitDefID(T[i])    
		if false== UnitDefs[def].isBuilder          then 
		returnTable[#returnTable+1]=T[i] end  
		end  
	return returnTable  
	end

--> filters Out Mobile Builders
    function filterOutMobileBuilder  (T)
   returnTable={}  
		for i=1,#T do    
		def=Spring.GetUnitDefID(T[i])    
		if false== UnitDefs[def].isMobileBuilder    then 
		returnTable[#returnTable+1]=T[i] end  
		end  
	return returnTable  
	end
	
    function filterOutStaticBuilder  (T)
   returnTable={}  
		for i=1,#T do    
		def=Spring.GetUnitDefID(T[i])    
		if false== UnitDefs[def].isStaticBuilder    then 
		returnTable[#returnTable+1]=T[i] end  
		end  
	return returnTable  
	end
	
    function filterOutFactory        (T)
   returnTable={}  
		for i=1,#T do    
		def=Spring.GetUnitDefID(T[i])    
		if false== UnitDefs[def].isFactory          then 
		returnTable[#returnTable+1]=T[i] end  
		end  
	return returnTable  
	end
	
    function filterOutExtractor      (T)
   returnTable={}  
		for i=1,#T do    
		def=Spring.GetUnitDefID(T[i])    
		if false== UnitDefs[def].isExtractor        then 
		returnTable[#returnTable+1]=T[i] end  
		end  
	return returnTable  
	end
   
   function filterOutGroundUnit     (T)
   returnTable={}  
		for i=1,#T do    
		def=Spring.GetUnitDefID(T[i])    
		if false== UnitDefs[def].isGroundUnit       then 
		returnTable[#returnTable+1]=T[i] end  
		end  
	return returnTable  
	end
   
   function filterOutAirUnit        (T)
   returnTable={}  
		for i=1,#T do    
		def=Spring.GetUnitDefID(T[i])    
		if false== UnitDefs[def].isAirUnit          then 
		returnTable[#returnTable+1]=T[i] end  
		end  
	return returnTable  
	end
    
	function filterOutStrafingAirUnit(T)
   returnTable={}  
		for i=1,#T do    
		def=Spring.GetUnitDefID(T[i])    
		if false== UnitDefs[def].isStrafingAirUnit  then 
		returnTable[#returnTable+1]=T[i] end  
		end  
	return returnTable  
	end
 
    function filterOutHoveringAirUnit(T)
   returnTable={}  
		for i=1,#T do    
		def=Spring.GetUnitDefID(T[i])    
		if false== UnitDefs[def].isHoveringAirUnit  then 
		returnTable[#returnTable+1]=T[i] end  
		end  
	return returnTable  
	end
	
    function filterOutFighterAirUnit (T)
   returnTable={}  
		for i=1,#T do    
		def=Spring.GetUnitDefID(T[i])    
		if false== UnitDefs[def].isFighterAirUnit   then 
		returnTable[#returnTable+1]=T[i] end  
		end  
	return returnTable  
	end
    
	function filterOutBomberAirUnit  (T)
   returnTable={}  
		for i=1,#T do    
		def=Spring.GetUnitDefID(T[i])    
		if false== UnitDefs[def].isBomberAirUnit    then 
		returnTable[#returnTable+1]=T[i]
		end  
		end  
	return returnTable  
	end

	 -->Spawn CEG at unit
	 function spawnCEGatUnit(unitID,cegname,xoffset,yoffset,zoffset)
	 x,y,z=Spring.GetUnitPosition(unitID)
			if xoffset then
			Spring.SpawnCEG(cegname,x+xoffset,y+yoffset,z+zoffset,0,1,0,50,0)
			else
			Spring.SpawnCEG(cegname,x,y,z,0,1,0,50,0)
			end
		end
		
	function funcyMeta (T, ...)
			 for _, f in pairs(arg) do
					 T=f(T)
			 end
	 return T
	 end
	 
	 --> Apply a function on a Table
	function forTableUseFunction(Table,func,metafunc)
	T={}
		for i=1,#Table do
		T[i]=func(Table[i])
		end
		if metafunc then
		return metafunc(T)
		else
		return T
		end
	end

	function getLowest(Table)
	lowest=0
	val=0
		for k,v in pairs(Table) do
			if v < val then 
			val=v
			lowest=k
			end
		end
	end

	function sumTable(Table)
	a=0
		for i=1,#Table do
		a=a+Table[i]
		end
	return a
	end

	function sumTableKV(Table)
	a=0
		for k,v in pairs(Table) do
		a=a+v
		end
	return a
	end

	function PieceLight(unitID, piecename,cegname)
		while true do
		x,y,z=Spring.GetUnitPiecePosDir(unitID,piecename)
		Spring.SpawnCEG(cegname,x,y+10,z,0,1,0,50,0)
		Sleep(250)
		end
	end
	
	function numTabletokeyTable(T)
	reT={}
	for i=1,#T do
	reT[T[i]]=T[i]
	end
	return reT
	end
		
	function TableMergeTable(TA,TB)
	T={}
		if #TA >= #TB then
		T=numTabletokeyTable(TA)
		
			for i=1,#TB do
				if not T[TB[i]] then
				TA[#TA+1]=TB[i]
				end
			end
		return  TA
		else
		T=numTabletokeyTable(TB)
			for i=1,#TA do
				if not T[TA[i]] then
				TB[#TB+1]=TA[i]
				end
			end
		return  TB
		end
		
	end
	

	function filterUnitTableforDefIDTable(unitIDT,KeyDefIDT)
	Tid={}
		for i=1,#unitIDT do 
		a=Spring.GetUnitDefID(unitIDT[i])
			if KeyDefIDT[a] then
			Tid[#Tid+1]=unitIDT[i]
			end
		end
	return Tid
	end

--> Forms a Tree from handed over Table
--	this function needs a global Itterator and but is threadsafe, as in only one per unit
	--	it calls itself, waits for all other threads running parallel to reach the same recursion Depth
	-- 	once hitting the UpperLimit it ends
	function executeLindeMayerSystem( gramarTable,String, oldDeg, Degree , UpperLimit, recursionDepth,recursiveItterator,PredPiece)
			
			-- this keeps all recursive steps on the same level waiting for the last one - who releases the herd
			gainLock(recursiveItterator)
					
			--we made it into the next step and get our piece
			GlobalTotalItterator=GlobalTotalItterator+1
			local hit=GlobalTotalItterator
			
			if not hit or TreePiece[hit] == nil or hit > UpperLimit  then 
			releaseLocalLock(recursiveItterator)
			return 
			end
		
		
			--DebugInfo
			--Spring.Echo("Level "..recursiveItterator.." Thread waiting for Level "..(recursiveItterator-1).. " to go from ".. GlobalInCurrentStep[recursiveItterator-1].." to zero so String:"..String.." can be processed")
				
			while GlobalInCurrentStep[recursiveItterator-1] > 0  do	
			Sleep(50)
			end

	--return clauses
		if not String or String == "" or string.len(String) < 1 or recursiveItterator == recursionDepth  then 
		RecursionEnd[#RecursionEnd+1]=PredPiece
		releaseLocalLock(recursiveItterator)
		return 
		end

	ox,oy,oz=Spring.GetUnitPiecePosition(unitID, PredPiece)

	--Move Piece to Position

	Show(TreePiece[hit])

	Move(TreePiece[hit],x_axis,ox,0)
	Move(TreePiece[hit],y_axis,oy,0)
	Move(TreePiece[hit],z_axis,oz ,0,true)
	--DebugStoreInfo[#DebugStoreInfo+1]={"RecursionStep:"..hit.." ||RecursionDepth: "..recursiveItterator.." ||String"..String.." ||PredPiece: "..PredPiece.."  || Moving Piece:"..TreePiece[hit].."to x:"..ox.." y:"..oy.." z:"..oz}

	--stores non-productive operators
	OperationStorage={}
	--stores Recursions and Operators
	RecursiveStorage={}
	
		  for i = 1, string.len(String) do
		  --extracting the next token form the current string and find out what type it is
		  token = string.sub(String,i,i)
		  typeOf=type(gramarTable.transitions[token])
				
				-- if the typeof is a function and a productive Element 
				if typeOf == 'function' and gramarTable.productiveSymbols[token] then 	
					--execute every operation so far pushed back into the operationStorage on the current piece
						
						for i=#OperationStorage,1, -1 do
						gramarTable.transitions[OperationStorage[i]](oldDeg,Degree,TreePiece[hit],PredPiece,recursiveItterator,recursiveItterator==UpperLimit-1)
						end
						
						WaitForTurn(TreePiece[hit],x_axis)
						WaitForTurn(TreePiece[hit],y_axis)
						WaitForTurn(TreePiece[hit],z_axis)
					--renewOperationStorage
					OperationStorage={}
					
					--This LindeMayerSystem has a go
						StartThread(executeLindeMayerSystem,
												 
												gramarTable,
												gramarTable.transitions[token](oldDeg ,Degree ,TreePiece[hit],PredPiece,recursiveItterator,recursiveItterator==UpperLimit-1),												
												oldDeg,
												Degree,
												UpperLimit,
												recursionDepth,
												recursiveItterator+1,
												EndPiece[math.max(1,math.min(#EndPiece,hit))]
												)
						
						--if we have a non productive function we push it back into the OperationStorage
						elseif typeOf=="function" then --we execute the commands on the current itteration-- which i
						OperationStorage[#OperationStorage+1]=token
						
							--recursionare pushed into the recursionstorage and executed after the current string has beenn pushed
							elseif typeOf== "string" and token == gramarTable.transitions["RECURSIONSTART"] then							
							--Here comes the trouble, we have to postpone the recurssion 
							RecursiveStorage[#RecursiveStorage+1],recursionEnd=extractRecursion(String,i,gramarTable.transitions["RECURSIONSTART"],gramarTable.transitions["RECURSIONEND"])
							i=math.min(recursionEnd+1,string.len(String))
							end
		
		end

		--Recursions are itterated last but not least
		if table.getn(RecursiveStorage) > 0 then
			for i=1,#RecursiveStorage do		
						

				StartThread(executeLindeMayerSystem,
										 
										gramarTable,
										RecursiveStorage[i],										
										oldDeg,
										Degree,
										UpperLimit,
										recursionDepth,
										recursiveItterator+1,
										EndPiece[math.max(1,math.min(#EndPiece,hit))]
										)
		
			end
		end
		--Recursion Lock - the last one steps the Global Itteration a level up
		
		releaseLocalLock(recursiveItterator)
		--Spring.Echo("Thread Level "..recursiveItterator.." signing off")
		return
end

	function assertAllArgs(...)
	if not arg then error("No arguments were given") return end
	nr=1
	for k,v in pairs(arg) do
	if not v then error("Argument Nr"..nr.." is nil "); return false end
	nr=1+nr
	end
	return true 
	end
		
	-->prepares large speaches for the release to the world
	function prep( Speach, Names,Limits, Alphas, DefaultSleepBylines)
	--if only Speach 
	if Speach and not Names and not Limits then return Speach end
	if not Speach or not Names or not Limits then return nil end
	Name       			=Names or "Dramatis Persona"
	Limit             	= Limits or 42
	Alpha           	= Alphas   or 0.9
	DefaultSleepByline	= DefaultSleepBylines or 750

	T={}
	itterator=1
	lineend=Limit
	size=string.len(Speach) or #Speach or Limit
	assert(size,"Size does matter")
	assert(Speach and type(Speach)=="string","Speach not of type string", Speach)
	assert(lineend and type(lineend)=="number","Limit not a number", Limit)
	assert(size and type(size)=="number","Limit not a number", Limit)



		while lineend < size do  

		lineend=string.find(Speach, "[^a-zA-Z0-9]", itterator+Limit)
		subString=string.sub(Speach,itterator,lineend)
			
			if subString then
			T[#T+1]={line= subString ,alpha=Alpha, name=Name, DelayByLine=DefaultSleepByline}
			else
			break
			end
			
			if not lineend then 
			break
			else
			itterator=lineend+1
			end
			assert(lineend)
			assert(size)
		end
		return T, true
	end


	--> Displays Text at UnitPos Thread
	 -->> Expects a table with Line "Text", a speaker Name "Text", a DelayByLine "Numeric", a Alpha from wich it will start decaying "Numeric"
	function say( TableOfLineAndT, redrawDelay, NameColour, TextColour,OptionString,UnitID)
	if type(TableOfLineAndT) == "string" then Spring.Echo(TableOfLineAndT) return end
	px,py,pz=0,0,0
	if not UnitID or Spring.ValidUnitID(UnitID)==false then
	return 
	end

	local LineNameTimeT= TableOfLineAndT

		--catching the case that there is not direct Unit speaking
		if type(UnitID)=="string" then 
		Spring.Echo(LineNameTimeT[1].name.. ": "..LineNameTimeT[i].line)
			if not 	LineNameTimeT[2].line then return end
			for i=2, #LineNameTimeT, 1 do
			Spring.Echo(LineNameTimeT[i].line)		
			end
		return
		end
		
	local spGetUnitPosition=Spring.GetUnitPosition	

		for i=1, #LineNameTimeT, 1 do
		
	spaceString=stringOfLength(" ",string.len(TableOfLineAndT[i].name))
		--Sleep time till next line
		px,py,pz=Spring.GetUnitPosition(UnitID)
		if not GG.Dialog then GG.Dialog ={} end
		GG.Dialog[#GG.Dialog+1]={frames=120, txt, x=px, y=py, z=pz}
		Sleep( LineNameTimeT[i].DelayByLine)
		end

	for i= #GG.Dialog, 1, -1 do
		if GG.Dialog[i].frames <=0 then table.remove(GG.Dialog,i) end	
	end

	end

	--> creates a table of pieces with name
	function makeTableOfNames(name,startnr,endnr)
	T={}
		for i=startnr, endnr, 1 do
		T[#T+1]=name..i	
		end
	return T
	end

	--> finds GenericNames and Creates Tables with them
	function GeneratePiecesTablebyNames(boolMakePiecesTable)

	piecesTable=Spring.GetUnitPieceList(unitID)
	TableByName={}
	NameAndNumber={}
		
			for i=1,#piecesTable,1 do
				s=string.reverse(piecesTable[i])
			
				for w in string.gmatch(s,"%d+") do
				if w then
					s=string.sub(s,string.len(w),string.len(s))
					NameAndNumber[i]={name=string.sub(piecesTable[i],1,string.len(piecesTable[i])-string.len(w)),
									  number=string.reverse(w)
									}
									
					if TableByName[	NameAndNumber[i].name] then TableByName[NameAndNumber[i].name] =TableByName[NameAndNumber[i].name] +1 else TableByName[NameAndNumber[i].name] =1 end
					break
								
				end
				end
			if not NameAndNumber[i] then NameAndNumber[i]={name=string.reverse(s)} end
			
			
			end
		
		for k,v in pairs(TableByName) do
			if v > 1 then
			Spring.Echo(k.. " = {}")
			end
		end
		
		
		for k,v in pairs(NameAndNumber) do
		
			if v and v.number then
			Spring.Echo(v.name..v.number .." = piece\""..v.name..v.number.."\"")
			Spring.Echo(v.name.."["..v.number.."]= "..v.name..v.number)
			else
			Spring.Echo(v.name.." = piece("..v.name..")")
			end
		end
		
		if boolMakePiecesTable and boolMakePiecesTable ==true then
		generatepiecesTableAndArrayCode(unitID)
		end
		
	end

	-->Drops a unitpiece towards the ground
	function PieceDropTillStop(unitID,piece,speedPerSecond, speedMax, bounceNr, boolSpinWhileYouDrop, bounceConstant,driftFunc)
	if not unitID or not piece or not speedPerSecond or not speedMax then return end
	bConstant= bounceConstant or 0.25
	speed=speedPerSecond or 0.5
	Drift= driftFunc or function (unitID,piece,x,y,z,time,speed) 
								dx,dy,dz =Spring.GetGroundNormal(x,z) 
								Move(piece,x_axis,dx*y* (1/time), speed)
								Move(piece,z_axis,dz*y* (1/time), speed)
								end

	if boolSpinWhileYouDrop and boolSpinWhileYouDrop==true then
	SpinAlongSmallestAxis(unitID,piece, math.random(-5,5),0.2)
	end
	if bounceNr then
	while bounceNr > 1 do

		x,y,z=Spring.GetUnitPiecePosDir(unitID,piece)
		time=0
		gh=Spring.GetGroundHeight(x,z)
		while speed < speedMax or y > gh do 
		Move(piece,y_axis, -y+gh,speed)
		time=time+1
		Sleep(1000)   
		x,y,z=Spring.GetUnitPiecePosDir(unitID,piece)
		Drift(unitID,piece,x,y,z,time,speed)
		gh=Spring.GetGroundHeight(x,z)
		speed=math.min(math.log(time+1)*4.905,speedMax)
		end
		
		upperLimit= y*bConstant
		x,y,z=Spring.GetUnitPiecePosDir(unitID,piece)
		y=y-gh
		time=0
		while speed > 1 and y < upperLimit do 
		Move(piece,y_axis,y+speed,speed)
		y=y+speed
		speed=speed/2
		Sleep(1000)
		time=time+1   
		x,_,z=Spring.GetUnitPiecePosDir(unitID,piece)
		Drift(unitID,piece,x,y,z,time,speed)
		end




	bounceNr=bounceNr-1
	end
		x,y,z=Spring.GetUnitPiecePosDir(unitID,piece)
		gh=Spring.GetGroundHeight(x,z)
		time=0
		speedMax=9.81
		
		while speed < speedMax or y > gh do 
		Move(piece,y_axis, -y+gh,speed)
		Sleep(1000)
		x,y,z=Spring.GetUnitPiecePosDir(unitID,piece)
		gh=Spring.GetGroundHeight(x,z)
		time=time+1   
		Drift(unitID,piece,x,y,z,time,speed)
		speed=math.min(math.log(time+1)*4.905,speedMax)
		end

	else 
	x,y,z=Spring.GetUnitPiecePosDir(unitID,piece)
	gh=Spring.GetGroundHeight(x,z)
	time=0
	speedMax=9.81
			
		
		while speed < speedMax or y > gh do 
		Move(piece,y_axis, -y+gh,speed)
		Sleep(1000)
		time=time+1
		x,y,z=Spring.GetUnitPiecePosDir(unitID,piece)
		gh=Spring.GetGroundHeight(x,z)   
		Drift(unitID,piece,x,y,z,time,speed)
		speed=math.min(math.log(time+1)*4.905,speedMax)
		end
	end
		if boolSpinWhileYouDrop and boolSpinWhileYouDrop==true then
		StopSpin(piece,x_axis,0)
		StopSpin(piece,y_axis,0)
		StopSpin(piece,z_axis,0)
		end
	x,y,z=Spring.GetUnitPiecePosDir(unitID,piece)
	MoveUnitPieceToGroundPos(unitID,piece,x,z,22, 5)
		
	end

	--> GetDistanceNearestEnemy
	function GetDistanceNearestEnemy(id)
	ed=Spring.GetUnitNearestEnemy(id)
	return GetUnitDistance(id,ed)
	end

	function holdsForAll(Var,fillterConditionString,...)
	if arg then
		for k,Val in pairs(arg) do
			if string.load("Var"..fillterConditionString.."Val")==false then return end
		end
	return true
	else
	return true
	end
	end

	function is(Var,fillterConditionString,...)
	f=string.load(fillterConditionString)
		if type(f)=="function" then
			for k,Val in pairs(arg) do
				if( f(Var,Val)==true )then return true end
			end
			
		else
			
			for k,Val in pairs(arg) do
				if string.load("Var"..fillterConditionString.."Val")==true then return true end
			end
			
			return false
		end
	end

	function MovePieceInRelation(piece,axis,distance, speed, worldCoordPiece)
	--move Vector
	dx,dy,dz=Spring.UnitScript.GetPieceRotation(worldCoordPiece)
	V={}

	if axis== "x_axis" then
	V[4]=distance
	elseif axis== "y_axis" then
	V[8]=distance
	else --z_axis 
	V[12]=distance
	end


	--make a counter rotation matrice
	V=RotationMatrice

	end

	function makeNewAffirmativeMatrice()
	V={	[1]=1,	[2]=	0,	[3]=	0,[4]=	0,
		[5]=0,	[6]=	1,	[7]=	0,[8]=	0,
		[9]=0,	[10]=	0,	[11]=	1,[12]=	0,
		[13]=0,	[14]=	0,	[15]=	0,[16]=	1	
		}
		function V.Mul(other)
		V[1]	= 0;	V[2]	=	0;	V[3] 	=	0;V[4]		=0	;
		V[5]	= 0;	V[6]	=	0;	V[7] 	=	0;V[8]		=0	;
		V[9]	= 0;	V[10]	=	0;	V[11]	=	0;V[12]	=0	;
		V[13]	= 0;	V[14]	=	0;	V[15]	=	0;V[16]	=0	
		end	
	--TODO
	--http://springrts.com/phpbb/viewtopic.php?f=21&t=32246
	return V
	end



	function makeAffirmativeRotationMatrice(axis, deg)
	V=makeNewAffirmativeMatrice()
		if axis=="x_axis" then
		V[6]=math.cos(-deg)	
		V[7]=-1*math.sin(-deg)	
		V[10]=math.sin(-deg)
		V[11]=math.cos(-deg)	
		
		elseif axis== "y_axis" then
		V[1]=math.cos(-deg)	
		V[3]=math.sin(-deg)	
		V[9]=-1*math.sin(-deg)
		V[11]=math.cos(-deg)	
		else
		V[1]=math.cos(-deg)	
		V[5]=math.sin(-deg)	
		V[2]=-1*math.sin(-deg)
		V[6]=math.cos(-deg)
		end
	return V	
	end

	-->Returns a Unit from the Game without killing it
	function removeFromWorld(unit,offx,offy,offz)
	--TODO - keepStates in general and commandqueu
	pox,poy,poz=Spring.GetUnitPosition(unit)
	Spring.SetUnitAlwaysVisible(unit,false)
	Spring.SetUnitBlocking(unit,false,false,false)
	Spring.SetUnitNoSelect(unit,true)
	Spring.MoveCtrl.Enable(unit,true)
		if offx then
		Spring.SetUnitPosition(unit,offx,offy,offz)
		end
	end
	-->Removes a Unit from the Game without killing it
	function returnToWorld(unit,px,py,pz)
	Spring.MoveCtrl.Enable(unit,false)
		if pz then
		Spring.SetUnitPosition(unit,px,py,pz)
		end
	Spring.SetUnitAlwaysVisible(unit,true)
	Spring.SetUnitBlocking(unit,true,true,true)
	Spring.SetUnitNoSelect(unit,false)
	end

	function MultiplyAffirmativeMatrice(self, other)
		

	end
	
	
	function SpinAlongSmallestAxis(unitID,piecename,degree, speed)
	if not piecename then return end
	vx,vy,vz=Spring.GetUnitPieceCollisionVolumeData(unitID,piecename)
	if vx and vy and vz then
	areax,areay,areaz=vy*vz,vx*vz,vy*vx
	end

	if holdsForAll(areax, " <= ", areay, areaz)then Spin(piecename,x_axis,math.rad(degree), speed) return end
	if holdsForAll(areay, " <= ", areaz, areax)then Spin(piecename,y_axis,math.rad(degree), speed) return end
	if holdsForAll(areaz, " <= ", areay, areax)then Spin(piecename,z_axis,math.rad(degree), speed) return end
	end

	 -->Helperfunction of recursiveAddTable
	function returnPieceChildrenTable(piecename,piecetable)
	if not piecename then return end
	T=Spring.GetUnitPieceInfo(unitID,piecename)
	children=T.children
	if children then
	for i=1,#children do children[i]=piecetable[children[i]] end
	end
	return children, T.max
	end

	--Hashmap of pieces --> with accumulated Weight in every Node
	--> Every Node also holds a bendLimits which defaults to ux=-45 x=45, uy=-180 y=180,uz=-45 z=45
	function recursiveAddTable(T,piecename,parent,piecetable)
		if not piecename then return T, 0 end

		C, max=returnPieceChildrenTable(piecename,piecetable)


	if not T[parent] then T[parent]={} end
		
		if C and #C > 0 then
			for i=1,#C do
			T,nr=recursiveAddTable(T,C[i],piecename,piecetable)
			end
		bendLimits=computateBendLimits(piecename,parent)
		T[parent].bendLimits=bendLimits
		T[parent].weight=max[1]*max[2]*max[3]
		T[parent].nr=#C
		else
		
		if not T[parent][piecename] then T[parent][piecename] ={} end
		computateBendLimits(piecename,parent)
		T[parent].bendLimits=bendLimits
		T[parent][piecename].weight=1
		end
	return T
	end

	--> finds the degree in a triangle where only the lenght of two sides are known
	function triAngleTwoSided(LowerSide, OpposingSide)
	norm= math.sqrt(LowerSide*LowerSide +OpposingSide*OpposingSide)
	return math.atan2(LowerSide/norm,OpposingSide/norm)
	end
	
	function getCubeSphereRad(x,y,z)
	xy,xz,yz=x*y,x*z,y*z
	
	if xy > xz and xy > yz then return math.sqrt(x*x +y*y) end
	
	if xz > xy and xz > yz then return math.sqrt(x*x +z*z) end
	
	if yz > xy and yz > xz then return math.sqrt(y*y +z*z) end
	end
	
	function computateBendLimits(piecename,parent)
	paPosX,paPosY,paPosZ=Spring.GetUnitPiecePosition(unitID,parent)
	cPosX,cPosY,cPosZ=Spring.GetUnitPiecePosition(unitID,piecename)

	--the offset of the piece in relation to its parentpiece
	v={}
	v.x,v.y,v.z=cPosX-paPosX,cPosY-paPosY,cPosZ-paPosZ

	pax,pay,paz=Spring.GetUnitPieceCollisionVolumeData(unitID,parent)
	radOfParentSphere=getCubeSphereRad(pax,pay,paz)
	cx,cy,cz=Spring.GetUnitPieceCollisionVolumeData(unitID,piecename)
	radOfPieceSphere=getCubeSphereRad(cx,cy,cz)
	-- rotate the vector so that it aligns with x,y,z origin vectors
	-- computate the orthogonal 
	-- computate the dead degree cube
	wsize=triAngleTwoSided(v.x,v.y)
	-- rotate the computated window inverse to the vector back
	-- voila
	
	--Y-Axis 
	-->TODO:RAGDOLL      |_\    -- you approximate the motherpiece with a circle and then do a  math.acos( circleradius/distance)
	--defaulting to a maxturn
	return {ux=-15 ,x=15, uy=-180, y=180,uz=-15, z=15}
	end

	function average(...)
	sum=0
	it=0
		for k,v in pairs(arg) do
		sum=sum+v
		it=it+1
		end
	return sum/it
	end

	-->Turns a piece towards its broadest side by Collissionvolume
	function turnPieceToLongestSide(unitID,pic,Speed)
	cv={}
	x,y,z=Spring.GetUnitPieceCollisionVolumeData(unitID,centerPiece)
	xy=x*y;yz=y*z;zx=z*x
	signum=math.random(-1,1)
	signum=signum/math.abs(signum)
	if xy > yz and xy> zx then tP(pic, 90*signum, 0,0, Speed); return z/2 end
	if yz > xy and yz> zx then tP(pic, 0, 0,90*signum, Speed); return x/2 end
	if zx > xy and zx> yz then tP(pic, 0, math.random(-360,360),0, Speed);return y/2  end
	end

	-->forges a hierachical ragdoll for the table of pieces for the given lifetime lets that ragdoll  hit the floor
	function ragDoll(tableOfPieces,centerPiece,fallSpeed,lifetime)
	deltaMovement=1
	cv={}
	PiecesMap={}
	FallSpeed=fallSpeed or 9.81
	cv.x,cv.y,cv.z=Spring.GetUnitPieceCollisionVolumeData(unitID,centerPiece)
	averageSize=average(cv.x,cv.y,cv.z)
	boolOnce=false;boolGo=false
		
		while lifetime > 0 do
		--we care for the centerPieceFalling
		px,py,pz=Spring.GetUnitPiecePosition(unitID,centerPiece)
		wpx,wpy,wpz=Spring.GetUnitPiecePosDir(unitID,centerPiece)
		h=Spring.GetGroundHeight(wpx,wpz)
		Dist= wpy-averageSize-FallSpeed/4;	if Dist < h then Dist =0; if boolGo== true then boolGo=false;boolOnce= true end end
			if boolOnce==true then averageSize=turnPieceToLongestSide(unitID,centerPiece,9.81/6);boolOnce=false end
			if py-h-averageSize/2 >0 then Move(centerPiece,y_axis,Dist,FallSpeed)end
		
		--and now we handle the other pieces going down
		subsOfCenterPiece=tableOfPieces[centerPiece]
			for _,v in pairs(subsOfCenterPiece) do
				traversePiecesMapToGround(centerPiece,v, PiecesMap, 9.81, 250)
			end
			
		Sleep(250)
		lifetime=lifetime-250
		end
	end

	-->MovesAParentPiece
	function movePieceAboveGround(parent, subPiece,Speed)

	local spGetUnitPiecePosDir=Spring.GetUnitPiecePosDir
	px,py,pz=spGetUnitPiecePosDir(unitID,subPiece)
	gh=Spring.GetGetGroundHeight(px,pz)
	--necessaryData
	paX,paY,paZ=spGetUnitPiecePosDir(unitID,parent)

		--TargetCoordsToTurnTowards
		TurnPieTowardsPoint (subPiece, px,gh+3,pz,Speed)

	end


	-->Traverses the PiecesMapDown recursively solving 
	function traversePiecesMapToGround(parentPiece,currentPiece, PiecesMap, Gravity, UpdateCycleLength)
	-- if the piece is underground it moves its parent to get itself above ground
	cX,cY,cZ=Spring.GetUnitPiecePosDir(unitID,currentPiece)
	if cY < Spring.GetGroundHeight(cX,cZ) then 

	-- if this Piece has subsidarys, it tryies to move itself to the ground, before calling for every children
		
		tP(currentPiece,tx,ty,tz,Gravity/(1000/UpdateCycleLength))

			for  _,UnitPiece in pairs(PiecesMap[parentPiece]) do
			end
		end
	end

	function stableConditon(legNr,q)
	return GG.MovementOS_Table~= nil and 
	GG.MovementOS_Table[unitID].stability >  0.5  and GG.MovementOS_Table[unitID].quadrantMap[math.max(math.min(4,q),1)] > 0 or  GG.MovementOS_Table[unitID].quadrantMap[math.max(math.min(4,legNr%2),1)] and  GG.MovementOS_Table[unitID].quadrantMap[math.max(math.min(4,legNr%2),1)]>0  
	end

	--Controlls One Feet- Relies on a Central Thread running and regular updates of each feet on his status
	function feetThread(quadrant,degOffSet, turnDeg, nr, FirstAxisPoint, KneeT, SensorPoint, Weight, Force,LiftFunction,LowerFunction,LegMax)
	LMax=LegMax or 5

		Sleep(500)
		
		lowerFeet(quadrant,degOffSet, turnDeg, nr, FirstAxisPoint, KneeT, SensorPoint, Weight, Force,LiftFunction,LowerFunction)
				
		while true do
			while GG.MovementOS_Table[unitID].boolmoving==true do
			
					while GG.MovementOS_Table[unitID].boolmoving==true and  stableConditon(nr,quadrant) do
					pushBody(quadrant,degOffSet, turnDeg, nr, FirstAxisPoint, KneeT, SensorPoint, Weight, Force,LiftFunction,LowerFunction)
				--feet go over knees if FeetLiftForce > totalWeight of Leg
					liftFeedForward(quadrant,degOffSet, turnDeg, nr, FirstAxisPoint, KneeT, SensorPoint, Weight, Force,LiftFunction,LowerFunction)
				--fall forward
				
				--catch
					lowerFeet(quadrant,degOffSet, turnDeg, nr, FirstAxisPoint, KneeT, SensorPoint, Weight, Force,LiftFunction,LowerFunction)
					Sleep(100)
					end
				--rebalance
				stabilize(quadrant,degOffSet, turnDeg, nr, FirstAxisPoint, KneeT, SensorPoint, Weight, Force,LiftFunction,LowerFunction)
			Sleep(100)
			end
		
				stabilize(quadrant,degOffSet, turnDeg, nr, FirstAxisPoint, KneeT, SensorPoint, Weight, Force,LiftFunction,LowerFunction)	
				Sleep(100)
		--setFeetInStableStance
		end
	end

	function pushBody(quadrant,degOffSet, turnDeg, nr, FirstAxisPoint, KneeT, SensorPoint, Weight, Force)
	Spring.Echo("pushBody")
	Turn(FirstAxisPoint,2,turnDeg)
		xp,yp,zp=Spring.GetUnitPiecePosDir(unitID,SensorPoint)
		dif=yp- Spring.GetGroundHeight(xp,zp)
		degToGo=0
		while (Spring.UnitScript.IsInTurn(FirstAxisPoint,2)==true) do
		
			xp,yp,zp=Spring.GetUnitPiecePosDir(unitID,SensorPoint)
			dif=yp- Spring.GetGetGroundHeight(xp,zp)
					if dif > 5 then degToGo=degToGo-1 else degToGo=degToGo+1 end
				
			Turn(FirstAxisPoint,1,math.rad(degToGo),Speed)

			WaitForTurn(FirstAxisPoint,1)
			Sleep(80)
		
		end

	end
	-->Uses the LiftAnimation Function to Lift the Feed
	function	liftFeedForward(quadrant,degOffSet, turnDeg, nr, FirstAxisPoint, KneeT, SensorPoint, Weight, Force,LiftFunction,LowerFunction)
	Spring.Echo("liftFeedForward")
	GG.MovementOS_Table[unitID].quadrantMap[quadrant%4+1]=GG.MovementOS_Table[unitID].quadrantMap[quadrant%4+1]-1
	LiftFunction(KneeT,Force/(#KneeT*Weight))

	WaitForTurn(KneeT[nr],2)
	Sleep(100)
	if degOffSet > 180 then turnDeg=turnDeg*-1 end
	Turn(FirstAxisPoint,2,math.rad(degOffSet+turnDeg), Force/(#KneeT*Weight))
	end
	-->Uses the Animation Function To Lower the Feet
	function	lowerFeet(quadrant,degOffSet, turnDeg, nr, FirstAxisPoint, KneeT, SensorPoint, Weight, Force,LiftFunction,LowerFunction)

		LowerFunction(KneeT,Force/(5*Weight),SensorPoint,FirstAxisPoint)
		GG.MovementOS_Table[unitID].quadrantMap[quadrant%4+1]=GG.MovementOS_Table[unitID].quadrantMap[quadrant%4+1]+1
	end
	-->Stabilizes the Feet
	function	stabilize(quadrant,degOffSet, turnDeg, nr, FirstAxisPoint, KneeT, SensorPoint, Weight, Force,LiftFunction,LowerFunction)
	Spring.Echo("stabilize")
		xp,yp,zp=Spring.GetUnitPiecePosDir(unitID,SensorPoint)
		dif=yp- Spring.GetGroundHeight(xp,zp)
		degToGo=0
		counter=0
		olddif=0
			while ( GG.MovementOS_Table[unitID].stability <= 0.5 ) do
				for i=1,#KneeT, 1 do
				x,y,z=Spring.GetUnitPiecePosDir(unitID,KneeT[i])
					if y > Spring.GetGroundHeight(x,z) then 
					deg=math.deg(Spring.UnitScript.GetPieceRotation(KneeT[i]))-1
					Turn(KneeT[i],x_axis,math.rad(deg), 0.3)
					else
					deg=math.deg(Spring.UnitScript.GetPieceRotation(KneeT[i]))+1
					Turn(KneeT[i],x_axis,math.rad(deg), 0.3)
					end
				end	
			GG.MovementOS_Table[unitID].stability=GG.MovementOS_Table[unitID].stability+0.02
			Sleep(50)
			end

	end


	--expects a Table containing:

	--unitID,centerNode,centerNodes, nrofLegs, FeetTable={firstAxisTable, KneeTable[nrOfLegs]},SensorTable,frameRate, FeetLiftForce
	--> Trys to create a animation using every piece there is as Legs.. 
	function adaptiveAnimation(configTable,inPeace)
	Spring.Echo("ToolKit::FixMe::adaptiveAnimation")
	local infoT= configTable
	pieceMap={}
	pieceMap[infoT.centerNode]={}
	pieceMap=recursiveAddTable(pieceMap,infoT.centerNode, infoT.centerNode,inPeace)
		if not GG.MovementOS_Table then GG.MovementOS_Table={} end
		if not GG.MovementOS_Table[unitID] then GG.MovementOS_Table[unitID]={} end

	maxDeg=math.random(12,32)
	turnOffset=360/#infoT.feetTable.Knees
			
			for i=1,infoT.nr do
			
			StartThread( feetThread,
						math.floor(math.min(math.max(0,(i*turnOffset)/360),1)*4),
						(90+(360/5)*i),
						maxDeg,
						i,
						infoT.feetTable.firstAxis[i],
						infoT.feetTable.Knees[i],
						infoT.sensorTable[i],
						infoT.ElementWeight,
						infoT.FeetLiftForce,
						infoT.LiftFunction,
						infoT.LowerFunction
						)
			end
		
		local spGetUnitPosition=Spring.GetUnitPosition
		local MotionDetect=	function (ox,oz)
		x,y,z=Spring.GetUnitPosition(unitID)
		return math.abs(ox-x) +math.abs(oz-z) < 15,x,z
		end
		
		quadrantMap={[1]=0,[2]=0,[3]=0,[4]=0}
		tx,ty,tz=spGetUnitPosition(unitID)

		GG.MovementOS_Table[unitID]={quadrantMap=quadrantMap,boolmoving=false, stability=1, tx=tx,ty=ty,tz=tz, ForwardVector={x=0,z=0}}
		Sleep(100)

		ox,oy,oz= spGetUnitPosition(unitID)
		boolMoving=false
		Height=infoT.Height
			while true do
			--find out whether we are moving
			ux,uz=ox,oz
			boolMoving,ox,oz=MotionDetect(ox,oz)
			GG.MovementOS_Table[unitID].tx=ox
			GG.MovementOS_Table[unitID].tz=oz
			
			GG.MovementOS_Table[unitID].boolmoving=(math.abs(ox-ux)+math.abs(z-uz))> 5
			GG.MovementOS_Table[unitID].ForwardVector={x=ox-ux,z=oz-uz}
			
			local one, three =GG.MovementOS_Table[unitID].quadrantMap[1],GG.MovementOS_Table[unitID].quadrantMap[3]
			local two,four   =GG.MovementOS_Table[unitID].quadrantMap[2],GG.MovementOS_Table[unitID].quadrantMap[4]
			total=one+two+three+four
			one,two,three,four=one>0,two> 0, three>0, four> 0
			--		//stabilityfactor
			BoolStable=((one and two and (three or four) )	)or 
						((two and four) and ( three or one) )or 
						((four and three) and ( one or two) )or
						((three and one ) and ( four or two))
						
				if BoolStable==false then
				GG.MovementOS_Table[unitID].stability=math.min(1,(1/total)*	GG.MovementOS_Table[unitID].stability)
				else
				GG.MovementOS_Table[unitID].stability=1			
				end
			
			Move(infoT.centerNode,y_axis,GG.MovementOS_Table[unitID].stability*Height,3)
			Sleep(400)	
			end
	end

	--> Initialises and generates a Square-Table 
	function prepSquareTable(size,defaultVal)
	Tab={}
	for i=1,size do
		Tab[i]={}
			for j=1,size do
				Tab[i][j]=defaultVal
			end
		end
	return Tab
	end


	function getADryWalkAbleSpot()
	min,max=Spring.GetGroundExtremes()
	if max <=0 then return end
	cond=function (i,j,chunkSizeX,chunkSizeZ)
	h=Spring.GetGroundHeight(i*chunkSizeX,chunkSizeZ*j)
			if h > 0 then 
			v={}
			v.x,v.y,v.z=Spring.GetGroundNormal(i*chunkSizeX,chunkSizeZ*j)
			v=normalizeVector(v)
				if v.y -math.abs(v.x)-math.abs(v.z) > 0.1 then
				return  math.ceil(i*chunkSizeX), math.ceil(i*chunkSizeZ) 
				end
			end
	end
	return getSpot(cond,64)
	end

	--> returns a randomized Signum
	function randSign()
	val=math.random(-1,1)
	return val/math.abs(val)
	end
	
	-->finds a spot on the map that is dry, and walkable
	function getSpot(condition,maxRes,filterTable)
	local lcondition =condition

	probeResolution=8
	local spGetGroundHeight=Spring.GetGroundHeight

		while true do
		
		chunkSizeX,chunkSizeZ=Game.mapSizeX/probeResolution,Game.mapSizeZ/probeResolution
		xRand,zRand=math.floor(math.random(1,probeResolution)),math.floor(math.random(1,probeResolution))
					
		for i=xRand,probeResolution,1 do
		for j=zRand,probeResolution,1 do
		ax,ay,az=	lcondition(i,j,chunkSizeX,chunkSizeZ,filterTable)
			if ax then return ax,ay,az end
		end
		end

		
		for i=1,xRand,1 do
		for j=1,zRand,1 do
		ax,ay,az=	lcondition(i,j,chunkSizeX,chunkSizeZ,filterTable)
		if ax then return ax,ay,az end
		end
		end
			
		probeResolution=probeResolution*2
		if probeResolution > maxRes then Spring.Echo("Aborting Due to High Probe Resolution"); return end
		end
	end

	-->CondtionFunction
	function GetSpot_condDeepSea(x,y,chunksizeX,chunksizeZ,filterTable)
	h=Spring.GetGroundHeight(x*chunkSizeX,y*chunkSizeZ)
	if h < filterTable.minBelow and h > filterTable.maxAbove then return x*chunkSizeX,y*chunkSizeZ end 

	end

	-->generates from Randomized squarefeeted blocks of size A and height B a Buildings
	function createRandomizedBuilding(Blocks, originPiece, cg,RepeaterFunc)
	--Reset Begin
	Move(originPiece,x_axis,0,0)
	Move(originPiece,y_axis,0,0)
	Move(originPiece,z_axis,0,0)

	
	
	for i=1,table.getn(Blocks),1 do
	Move(Blocks[i],x_axis,0,0)
	Move(Blocks[i],y_axis,0,0)
	Move(Blocks[i],z_axis,0,0)
	end
	
	hideT(Blocks)
	--Reset End
	--default repetition pattern
	repFunc=  function (itterator,modulator)
						return itterator %modulator>3 
						end 
						
	repeatPatternFonc= RepeaterFunc or repFunc
	--default dirTable
	dirTable=	{[0]=function(sizeA,sizeB) return 0,sizeB,0,0 end,
				 [1]=function(sizeA,sizeB) return sizeA,0,0,1 end,
				 [2]=function(sizeA,sizeB) return 0,0,sizeA,2 end,
				 [3]=function(sizeA,sizeB) return -1*sizeA,0,0,3 end,
				 [4]=function(sizeA,sizeB) return 0,0,-1*sizeA,4 end,
				 [5]=function(sizeA,sizeB) return 0,sizeB,0,5 end,
				 [6]=function(sizeA,sizeB) return 0,sizeB,0,6 end
				 }
	--default growthDirection Function
	growthDirFunc= function (i,piecename,lastDir,boolInit,itterator, modulator,repeatPatternFonc,dirTable)
						
			vx,vy,vz=Spring.GetUnitPieceCollisionVolumeData(unitID,piecename)
				if not vx then return 0,0,0,0 end
			sizeA=math.min(vx,vz)
			sizeB=math.sqrt(vy)
			if boolInit==false then
				if lastdir and math.random(0,3)== 2 then return dirTable[lastDir](sizeA,sizeB) end 
			if math.random(0,3) ~= 1 			then return 0,sizeB,0,0 end
			end
			
			if repeatPatternFonc(itterator, modulator)==true then
			return dirTable[lastDir](sizeA,sizeB)
			else
			return dirTable[math.ceil(math.random(1,#dirTable))](sizeA,sizeB)
			end
		
		end
		
	--adding the first positions to expand upon	
	ogPosX,ogPosY,ogPosZ=Spring.GetUnitPiecePosition(unitID,originPiece)
	cgPosX,cgPosY,cgPosZ=Spring.GetUnitPiecePosition(unitID,cg)
	cgPosX,cgPosY,cgPosZ=ogPosX-cgPosX,ogPosY-cgPosY,ogPosZ-cgPosZ
	
	gDirFunc=growTable or growthDirFunc
	freeSpots={}

	for i=0,5,1 do
	x,y,z=gDirFunc(i,originPiece,i,true,3,7,repeatPatternFonc,dirTable)
	freeSpots[i]={}
	freeSpots[i][1]={}
	freeSpots[i][1]=cgPosX+x
	freeSpots[i][2]={}
	freeSpots[i][2]=cgPosY+y
	freeSpots[i][3]={}
	freeSpots[i][3]=cgPosZ+z
	end
	
	--add Block by Block

	Spring.Echo("ToolKit::createRandomizedBuilding-FixMe")
		for i=1,#Blocks,1 do
		Sleep(500)
		if  Blocks[i] then 
		Show(Blocks[i])
		getAPod=math.floor(math.random(1,#freeSpots))
	
		nrBlok=math.floor(math.random(1,table.getn(Blocks)))
		nrFreeSpot=math.floor(math.random(1,#freeSpots))
		Blocks,freeSpots= moveBlockAddPod(	freeSpots[nrFreeSpot][1],
											freeSpots[nrFreeSpot][2],
											freeSpots[getAPod][3],	
											nrFreeSpot,
											nrBlok,
											Blocks,
											freeSpots,
											gDirFunc,
											repeatPatternFonc,
											dirTable,
											cg
											)
		end
		end
	end

	function moveBlockAddPod(x,y,z,nrFreeSpot,nrBlok,bloks,freeSpots,gDirFunc,repeatPatternFonc,dirTable,cg)
	cgPosX,cgPosY,cgPosZ = x,y,z

	Move(cg,x_axis,x,0)
	Move(cg,y_axis,y,0)
	Move(cg,z_axis,z,0)
	WaitForMove(cg,y_axis)
	ax,ay,az=Spring.GetUnitPiecePosition(unitID,cg)
	Move(bloks[nrBlok],x_axis,ax,0)
	Move(bloks[nrBlok],y_axis,ay,0)
	Move(bloks[nrBlok],z_axis,az,0)

	WaitForMove(bloks[nrBlok],y_axis)
	d=math.floor(math.random(0,3))*90
	Turn(bloks[nrBlok],y_axis,math.rad(d),0)
	--bx,by,bz=Spring.GetUnitBasePosition(unitID)
	LastPos=0
		for i=1,5,1 do --add the free spots
		
		Show(bloks[nrBlok])
		d=table.getn(freeSpots)+1
		x,y,z,LastPos=gDirFunc	(i,bloks[nrBlok],LastPos,false,nrBlok,7,repeatPatternFonc,dirTable)
		freeSpots[d]={}
		freeSpots[d][1]={}
		freeSpots[d][1]=cgPosX+x
		freeSpots[d][2]={}
		freeSpots[d][2]=cgPosY+y
		freeSpots[d][3]={}
		freeSpots[d][3]=cgPosZ+z
		end

	table.remove(bloks,nrBlok)
	table.remove(freeSpots,nrFreeSpot)

	return bloks, freeSpots
	end

	-->Sanitizes a Variable for a table
	function sanitizeItterator(Data,Min,Max)
	return math.max(Min,math.min(Max,Data))
	end
	
	-->Splits a Table into Two Pieces
	function splitTable(T,breakP)
	breakPoint=breakP or math.ceil(#T/2)
	breakPoint=sanitizeItterator(breakPoint,1,#T)
	T1,T2={},{}
		for i=1,breakPoint do
		T1[i]=T[i]
		end
		
		for i=math.min(breakPoint+1,#T),#T do
		T2[i-breakPoint]=T[i]
		end
	
	return T1,T2
	end
