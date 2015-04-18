function gadget:GetInfo()
  return {
    name      = "Dev Helper Cmds",
    desc      = "provides various luarules commands to help developers, can only be used after /cheat",
    author    = "Bluestone",
    date      = "05/04/2013",
    license   = "Horses",
    layer     = 0,
    enabled   = true  --  loaded by default?
  }
end

if (not gadgetHandler:IsSyncedCode()) then
	return
end

function Plop()
	if not Spring.IsCheatingEnabled() then return end

    -- give all units with a decent space between them...
    local x,z = 100,100
    local s = 250
    for uDID,_ in pairs(UnitDefs) do
        Spring.CreateUnit(uDID,x,Spring.GetGroundHeight(x,z),z,"n",0)
         
        x = x + s
        if (x>5*s) then   
            x = 100
            z = z + s
        end
    end
end



function gadget:Initialize()
	gadgetHandler:AddChatAction('plop', Plop, "")
end


function gadget:Shutdown()
	gadgetHandler:RemoveChatAction('plop')
end




--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
