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
local BUFFER = 1

local playingTime = 0
local trackTime

function widget:Initialize()
    Spring.PlaySoundStream("sounds/music.ogg", VOLUME)
    trackTime = Spring.GetSoundStreamTime()
end

-- sets status to ready & hide pre-game UI
function widget:Update(dt)
    playingTime = playingTime + dt
    if playingTime > trackTime then
        playingTime = playingTime - trackTime
    elseif playingTime > trackTime - BUFFER then
        Spring.PlaySoundStream("sounds/music.ogg", VOLUME, true)
    end
end 

function widget:Shutdown()
    Spring.StopSoundStream()
end