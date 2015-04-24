function widget:GetInfo()
  return {
    name      = "Command Colours",
    desc      = "Replace cmdcolors via epic haxxors because there is no Lua API for doing it cleanly, hint hint",
    author    = "Redacted",
    date      = "Redacted",
    license   = "This one might actually break stuff",
    layer     = 0,
    enabled   = true,
  }
end


local OriginalCmdColors = ""

function GetOriginalCmdColors()
    local f = io.open("cmdcolors.txt", "r")
    for line in f:lines() do
        OriginalCmdColors = OriginalCmdColors .. "line" .. "\n"
    end
    f:close()
end

function WriteOriginalCmdColors()
    local f = io.open("cmdcolors.txt", "w+")
    f:write(OriginalCmdColors)
    f:close()
end

local cmdColors = {
    -- selected units boxes
    "unitBoxLineWidth  1.49",
    "unitBox           0.0  1.0  0.0  1.0",
    "buildBox          0.0  1.0  0.0  1.0",
    "allyBuildBox      0.8  0.8  0.2  1.0",
    "buildBoxesOnShift 1",
    
    -- mouse selection box
    "mouseBoxLineWidth  1.49",
    "mouseBox           1.0  1.0  1.0  1.0",
    "mouseBoxBlendSrc   src_alpha",
    "mouseBoxBlendDst   one_minus_src_alpha",
    
    -- command drawing
    "alwaysDrawQueue   1",

    "useQueueIcons     1",
    "queueIconAlpha    0.5",   
    "queueIconScale    1.0",

    "useColorRestarts  1",
    "useRestartColor   0",
    "restartAlpha      1.0",

    "queuedLineWidth   1.49",
    "queuedBlendSrc    src_alpha",
    "queuedBlendDst    one_minus_src_alpha",
    "stipplePattern    0xffcc",
    "stippleFactor     1",
    "stippleSpeed      1.0",
    
    "start       0.0  0.0  0.0  0.0",
    "restart     0.0  0.0  0.0  0.0",

    "stop        0.0  0.0  0.0  0.0",
    "wait        0.0  0.0  0.0  0.0",
    "build       0.0  0.0  0.0  0.0",
    "move        0.5  1.0  0.5  0.7", -- we don't draw any other engine commands; the "attack" command is drawn by target on the move (its a settarget command)
    "attack      0.0  0.0  0.0  0.0",
    "fight       0.0  0.0  0.0  0.0",
    "guard       0.0  0.0  0.0  0.0",
    "patrol      0.0  0.0  0.0  0.0",
    "capture     0.0  0.0  0.0  0.0",
    "repair      0.0  0.0  0.0  0.0",
    "reclaim     0.0  0.0  0.0  0.0",
    "restore     0.0  0.0  0.0  0.0",
    "resurrect   0.0  0.0  0.0  0.0",
    "load        0.0  0.0  0.0  0.0",
    "unload      0.0  0.0  0.0  0.0",
    "deathWatch  0.0  0.0  0.0  0.0",
    
    -- range drawing
    "selectedLineWidth  1.49",
    "selectedBlendSrc   src_alpha",
    "selectedBlendDst   one_minus_src_alpha",

    "rangeAttack          1.0  0.3  0.3  0.7",
    "rangeBuild           0.3  1.0  0.3  0.7",
    "rangeRadar           0.3  1.0  0.3  0.7",
    "rangeSonar           0.3  0.3  1.0  0.7",
    "rangeSeismic         0.8  0.1  0.8  0.7",
    "rangeJammer          1.0  0.3  0.3  0.7",
    "rangeSonarJammer     1.0  0.3  0.3  0.7",
    "rangeShield          0.8  0.8  0.3  0.7",
    "rangeDecloak         0.3  0.3  1.0  0.7",
    "rangeExtract         1.0  0.3  0.3  0.7",
    "rangeKamikaze        0.8  0.8  0.1  0.7",
    "rangeSelfDestruct    0.8  0.1  0.1  0.7",
    "rangeInterceptorOn   1.0  1.0  1.0  0.7",
    "rangeInterceptorOff  0.0  0.0  0.0  0.7",
}



function widget:Initialize()
    -- save the original cmdcolors.txt into a string
    GetOriginalCmdColors()
    
    -- edit cmdcolors.txt, then reload it
    local f = io.open("cmdcolors.txt","w+") -- erase file, open for writing
    for _,line in ipairs(cmdColors) do
        f:write(line .. "\n")
    end
    f:close()
    
    Spring.SendCommands("cmdcolors")
end

function widget:ShutDown()
    -- write the original cmdcolors.txt back
    -- user is screwed if this crashes, lol
    WriteOriginalCmdColors()    
end
