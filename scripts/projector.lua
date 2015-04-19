	include "toolKit.lua"

	piecesTable={}
	 piecesTable[#piecesTable+1]= piece("center")
	 piecesTable[#piecesTable+1]= piece("Body")
	 piecesTable[#piecesTable+1]= piece("legfront02")
	 piecesTable[#piecesTable+1]= piece("frblade02")
	 piecesTable[#piecesTable+1]= piece("legrear02")
	 piecesTable[#piecesTable+1]= piece("rblade02")
	 piecesTable[#piecesTable+1]= piece("legrear01")
	 piecesTable[#piecesTable+1]= piece("rblade01")
	 piecesTable[#piecesTable+1]= piece("legfront01")
	 piecesTable[#piecesTable+1]= piece("frblade01")
	 piecesTable[#piecesTable+1]= piece("armup01")
	 piecesTable[#piecesTable+1]= piece("MG01")
	 piecesTable[#piecesTable+1]= piece("MG_B01")
	 piecesTable[#piecesTable+1]= piece("bladeup01")
	 piecesTable[#piecesTable+1]= piece("rocket01")
	 piecesTable[#piecesTable+1]= piece("armup02")
	 piecesTable[#piecesTable+1]= piece("MG02")
	 piecesTable[#piecesTable+1]= piece("MG_B02")
	 piecesTable[#piecesTable+1]= piece("bladeup02")
	 piecesTable[#piecesTable+1]= piece("rocket02")
	 piecesTable[#piecesTable+1]= piece("turret")
	 piecesTable[#piecesTable+1]= piece("gun01")
	 piecesTable[#piecesTable+1]= piece("gun02")
	 piecesTable[#piecesTable+1]= piece("gun03")
	 piecesTable[#piecesTable+1]= piece("gun04")
	center=piece("center")
	Body=piece("Body")

	legfront02=piece("legfront02")
	frblade02=piece("frblade02")
	legrear02=piece("legrear02")
	rblade02=piece("rblade02")
	legrear01=piece("legrear01")
	rblade01=piece("rblade01")
	legfront01=piece("legfront01")
	frblade01=piece("frblade01")
	armup01=piece("armup01")
	bladeup01=piece("bladeup01")
	armup02=piece("armup02")
	bladeup02=piece("bladeup02")

	rocket01=piece("rocket01")
	MG02=piece("MG02")
	MG_B02=piece("MG_B02")
	MG01=piece("MG01")
	MG_B01=piece("MG_B01")
	rocket02=piece("rocket02")
	turret=piece("turret")
	gun01=piece("gun01")
	gun02=piece("gun02")
	gun03=piece("gun03")
	gun04=piece("gun04")

	boolStunned=false
	boolAiming=false

	function script.HitByWeapon ( x, z, weaponDefID, damage ) 
	StartThread(stunDetect)
	end
	
	
	
	function stunDetect()
	Sleep(50)
	  health,   maxHealth,   paralyzeDamage,   captureProgress,   buildProgress=Spring.GetUnitHealth(unitID)
		while health < paralyzeDamage do
		boolStunned=true
		Sleep(500)
		health,   maxHealth,   paralyzeDamage,   captureProgress,   buildProgress=Spring.GetUnitHealth(unitID)
		end

	end


	function stunned()
		decrease=44
		while decrease > 1 do
		--contract spasm
		tP(legfront02,math.random(-10,10),0,0,5)
		tP(legfront01,math.random(-10,10),0,0,5)
		tP(legrear01,math.random(-10,10),0,0,5)
		tP(legrear02,math.random(-10,10),0,0,5,true)	
		
			tP(legfront01,x_axis,-1*decrease,5)
			tP(legfront02,x_axis,-1*decrease,5)
			tP(frblade01,x_axis,decrease,5)
			tP(frblade02,x_axis,decrease,5)
			tP(legrear01,x_axis,decrease,5)
			tP(legrear02,x_axis,decrease,5)
			tP(rblade01,x_axis,-1*decrease,5)
			tP(rblade02,x_axis,-1*decrease,5)

		decrease=decrease*-1
			if maRa()==true then
				for i=1, 5 do
				tP(frblade01,math.random(-10,10)+decrease,0,0,5)
				tP(frblade02,math.random(-10,10)+decrease,0,0,5)
				tP(rblade01,math.random(-10,10)+decrease,0,0,5)
				tP(rblade02,math.random(-10,10)+decrease,0,0,5)
				end
			end

		--release
		equiTurn(bladeup01,armup01,x_axis,math.random(decrease,-1*decrease),4.5)
		equiTurn(bladeup02,armup02,x_axis,math.random(decrease,-1*decrease),4.5)
		equiTurn(frblade02,legfront02,x_axis,math.random(-10,0)+decrease,4.5)
		equiTurn(frblade01,legfront01,x_axis,math.random(-10,0)+decrease,4.5)
		equiTurn(legrear01,rblade01,x_axis,math.random(-10,0)+decrease,4.5)
		equiTurn(legrear02,rblade02,x_axis,math.random(-10,0)+decrease,4.5)
		Sleep(500)
		
		tP(legfront02,math.random(-10,10),0,0,5)
		tP(legfront01,math.random(-10,10),0,0,5)
		tP(legrear01,math.random(-10,10),0,0,5)
		tP(legrear02,math.random(-10,10),0,0,5,true)	
		
		decrease=decrease*-1
		decrease=decrease/2
		Sleep(100)
		end
		
		piq=3.14159*0.45
			while boolStunned==true do
			temp_O_Rary=math.random(11,11.5)*piq
				waveATable(piecesTable, x_axis,  math.sin, 1, 0.14, 6.2831853071,8.5, false,temp_O_Rary)
			Sleep(1000)
			end
		reseT(piecesTable,15)

	end


	function walkAnimation()


		while true do
				while boolMove==true do
					Sleep(100)
					tP(legfront01,-15,-33,0, 5,false)
					tP(frblade01,64,0,0, 5,false)
					tP(legrear02,-59,0,0, 5,false)
					tP(rblade02,89,0,0, 5,false)
					resetPiece(legfront02,5)
					resetPiece(frblade02,5)
					resetPiece(legrear01,5)
					resetPiece(rblade01,5,true)
					
					if boolAiming==false then
					tP(armup01,-5,0,0, 0.25,false)
					tP(armup02,-5,0,0, 0.25,false)	
					end
					Sleep(100)
					tP(legfront02,-15,-33,0, 5,false)
					tP(frblade02,64,0,0, 5,false)
					tP(legrear01,-59,0,0, 5,false)
					tP(rblade01,89,0,0, 5,false)
					resetPiece(legfront01,5)
					resetPiece(frblade01,5)
					resetPiece(legrear02,5)
					resetPiece(rblade02,5,true)
					
					if boolAiming==false then
					tP(armup01,15,0,0, 0.25,false)
					tP(armup02,15,0,0, 0.25,false)	
					end

					while boolStunned==true do
					stunned()
					end
				end
			
				
				
				while boolMove==false do
				
				--idle

				tP(legfront01,-4,0,0, 0.25,false)
				tP(legfront02,-4,0,0, 0.25,false)
				tP(frblade01,4,0,0, 0.25,false)
				tP(frblade02,4,0,0, 0.25,false)
				
				tP(legrear01,4,0,0, 0.25,false)
				tP(legrear02,4,0,0, 0.25,false)
				tP(rblade01,-4,0,0, 0.25,false)
				tP(rblade02,-4,0,0, 0.25,true)
				if boolAiming==false then
					tP(armup01,5,0,0, 0.05,false)
					tP(armup02,5,0,0, 0.05,false)	
				end
				Move(center,y_axis,-5,2)
				Sleep(100)
				tP(legfront01,-4*-1,0,0, 0.25,false)
				tP(legfront02,-4*-1,0,0, 0.25,false)
				tP(frblade01,4*-1,0,0, 0.25,false)
				tP(frblade02,4*-1,0,0, 0.25,false)
				
				tP(legrear01,4*-1,0,0, 0.25,false)
				tP(legrear02,4*-1,0,0, 0.25,false)
				tP(rblade01,-4*-1,0,0, 0.25,false)
				tP(rblade02,-4*-1,0,0, 0.25,true)
				if boolAiming==false then
					tP(armup01,10,0,0, 0.05,false)
					tP(armup02,10,0,0, 0.05,false)	
				end
				Move(center,y_axis,0,1)
				Sleep(100)
					while boolStunned==true do
					stunned()
					end
				end


			Sleep(100)
		end



	end

	boolRocketType	=false
	boolMGType		=false
	boolRay1Type		=false

	--Here we specify what type of robot we got
	function typeDef()
	defID=Spring.GetUnitDefID(unitID)

		if defID == UnitDefNames["wallier"].id then
		boolRocketType=true
		Show(rocket01)
		Show(rocket02)
		elseif  defID == UnitDefNames["bob"].id then
		Show(MG01)
		Show(MG02)
		Show(MG_B01)
		Show(MG_B02)
		boolMGType=true
		elseif  defID == UnitDefNames["projector"].id then
		boolRay1Type=true
		Show(turret)
		Show(gun01)
		else
		Spring.Echo("Projector is of unknown type")
		end




	end

	 
	function script.Create()
	Hide(MG01    )               
	Hide(MG02    )               
	Hide(MG_B01  )               
	Hide(MG_B02  )               
	Hide(rocket01)               
	Hide(rocket02)               
	Hide(turret  )               
	Hide(gun01   )               
	Hide(gun02   )               
	Hide(gun03   )               
	Hide(gun04   )  
	typeDef()

	StartThread(walkAnimation)
             
	end


	function script.StartMoving()
	boolMove=true
	end

	function script.StopMoving()
	boolMove=false		
			
	end

	function script.Activate()

	return 1
	end

	function script.Deactivate()

	return 0
	end

	function script.QueryBuildInfo() 
	  return center 
	end

	function script.QueryNanoPiece()
		 return center
	end
	
	function DeathAnimation()
	tP(legfront01,x_axis,-44,3)
	tP(legfront02,x_axis,-44,3)
	tP(frblade01,x_axis,44,3)
	tP(frblade02,x_axis,44,3)
	tP(legrear01,x_axis,44,3)
	tP(legrear02,x_axis,44,3)
	tP(rblade01,x_axis,-44,3)
	tP(rblade02,x_axis,-44,3)
	tP(center,z_axis,-90,8,true)

	end

	function script.Killed(recentDamage,_)


	DeathAnimation()
	return 1
	end
	SIG_RESET=2

	function resetTurret()
	Sleep(1000)
	SetSignalMask(SIG_RESET)
	resetPiece(turret,5)
	
	end


	----Weapon1
	function script.AimFromWeapon1() return rocket01 end

	function script.QueryWeapon1() 	return rocket01 end

	function script.AimWeapon1( Heading ,pitch)	
		if boolRocketType==false then return false end
		Signal(SIG_RESET)
		boolAiming=true
		
		Turn(armup01,x_axis,pitch,19)
		Turn(turret,y_axis,-Heading,19)
		Turn(armup02,x_axis,pitch,19,true)
		return boolRocketType
	end
	 
	 function script.FireWeapon1()	boolAiming=false; Signal(SIG_RESET); StartThread(resetTurret);return true; end


	----Weapon2
	function script.AimFromWeapon2() return gun01 end

	function script.QueryWeapon2() 	return gun01 end

	function script.AimWeapon2( Heading ,pitch)	
		if boolRay1Type==false then return false end
		Signal(SIG_RESET)
		boolAiming=true
		
		Turn(armup01,x_axis,pitch,19)
		Turn(turret,y_axis,-Heading,19)
		Turn(armup02,x_axis,pitch,19,true)
		return boolRay1Type
	end
	 
	 function script.FireWeapon2()	boolAiming=false; Signal(SIG_RESET); StartThread(resetTurret);return true; end


	----Weapon1
	function script.AimFromWeapon3() return MG01 end

	function script.QueryWeapon3() 	return MG01 end
	
	function ReloadMG()
	Sleep(5000)
	boolMGLoaded=true
		StopSpin(MG_B01,z_axis,0.15)
		StopSpin(MG_B02,z_axis,0.15)
	end
	quartPI_Innt= 8192
	boolMGLoaded=true
	function script.AimWeapon3( Heading ,pitch)	
		if boolRay2Type==false then return false end
		Signal(SIG_RESET)
		Spin(MG_B01,z_axis,5,5)
		Spin(MG_B02,z_axis,5,5)
		boolAiming=true
		
		Turn(armup01,x_axis,pitch-8192,19)
		Turn(turret,y_axis,-Heading,19)
		Turn(armup02,x_axis,pitch-8192,19,true)
		return boolMGType and boolMGLoaded
	end
	 
	 function script.FireWeapon3()	
	boolMGLoaded=false
	 boolAiming=false
	 Signal(SIG_RESET); StartThread(resetTurret); 
	 StartThread(ReloadMG)
	 
	 return true; 
	 
	 end

