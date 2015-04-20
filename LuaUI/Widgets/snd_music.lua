function widget:GetInfo()
  return {
    name      = "Music for dummies",
    desc      = "",
    author    = "ashdnazg",
    date      = "yesterday",
    license   = "GPL-v2",
    layer     = 1001,
    enabled   = true,
  }
end

local VOLUME = 0.3
local BUFFER = 0.015

local playingTime = 0
local trackTime
local gameStarted = false

function widget:GameStart()
    Spring.PlaySoundStream("sounds/music.ogg", VOLUME)
    _, trackTime = Spring.GetSoundStreamTime()
    gameStarted = true
end


function widget:Update(dt)
    if gameStarted then
        playingTime = Spring.GetSoundStreamTime()
        if playingTime > trackTime - BUFFER then
            Spring.PlaySoundStream("sounds/music.ogg", VOLUME)
        end
    end
end 

function widget:Shutdown()
    Spring.StopSoundStream()
end