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
MG01=piece("MG01")
MG_B01=piece("MG_B01")
bladeup01=piece("bladeup01")
rocket01=piece("rocket01")
armup02=piece("armup02")
MG02=piece("MG02")
MG_B02=piece("MG_B02")
bladeup02=piece("bladeup02")
rocket02=piece("rocket02")
turret=piece("turret")
gun01=piece("gun01")
gun02=piece("gun02")
gun03=piece("gun03")
gun04=piece("gun04")


boolAiming=false

function walkAnimation()

    while true do
        while boolMove == true do
            Sleep(100)
            tP(legfront01,-15,-33,0, 5,false)
            tP(frblade01,64,0,0, 5,false)
            tP(legrear02,-59,0,0, 5,false)
            tP(rblade02,89,0,0, 5,false)
            resetPiece(legfront02,5)
            resetPiece(frblade02,5)
            resetPiece(legrear01,5)
            resetPiece(rblade01,5,true)

            if boolAiming == false then
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

            if boolAiming == false then
                tP(armup01,15,0,0, 0.25,false)
                tP(armup02,15,0,0, 0.25,false)	
            end

        end
        resetPiece(legfront02,5,false)
        resetPiece(legfront01,5,false)
        resetPiece(frblade02,5 ,false)
        resetPiece(frblade01,5 ,false)
        resetPiece(legrear01,5 ,false)
        resetPiece(legrear02,5 ,true)

        while boolMove == false do

            --idle

            tP(legfront01,-4,0,0, 0.25,false)
            tP(legfront02,-4,0,0, 0.25,false)
            tP(frblade01,4,0,0, 0.25,false)
            tP(frblade02,4,0,0, 0.25,false)

            tP(legrear01,4,0,0, 0.25,false)
            tP(legrear02,4,0,0, 0.25,false)
            tP(rblade01,-4,0,0, 0.25,false)
            tP(rblade02,-4,0,0, 0.25,true)
            tP(armup01,5,0,0, 0.05,false)
            tP(armup02,5,0,0, 0.05,false)	
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

            tP(armup01,10,0,0, 0.05,false)
            tP(armup02,10,0,0, 0.05,false)	
            Move(center,y_axis,0,1)
            Sleep(100)
        end

        Sleep(100)
    end

end

function script.HitByWeapon(x, z, weaponDefID, damage) 
end
 
function script.Create()
    StartThread(walkAnimation)
end

function script.Killed(recentDamage, _)
    --suddenDeathV(recentDamage)
    return 1
end


----aimining & fire weapon
function script.AimFromWeapon1() 
    return center 
end


function script.QueryWeapon1() 
    return turret
end

function script.AimWeapon1(heading, pitch)	
    --aiming animation: instantly turn the gun towards the enemy
--     boolAiming=true
--     tP(armup01, pitch, 12)
--     tP(turret, 0, -heading, 0, 12)
--     tP(armup02, pitch, 12, true)
    return true

end
 

function script.FireWeapon1()
    boolAiming = false
    resetPiece(turret, 5)
    return true
end



function script.StartMoving()
    boolMove = true
end

function script.StopMoving()
    boolMove = false
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

